use crate::{
    adaptors::CacheMe,
    dispatching::{
        shutdown_check_timeout_for, shutdown_inner, stop_token::StopToken, update_listeners,
        update_listeners::UpdateListener, DispatcherState, ShutdownToken,
    },
    error_handlers::{ErrorHandler, LoggingErrorHandler},
    requests::Requester,
    types::{AllowedUpdate, Update},
};
use dptree::di::DependencyMap;
use futures::StreamExt;
use std::{collections::HashSet, convert::Infallible, fmt::Debug, ops::ControlFlow, sync::Arc};
use teloxide_core::requests::{Request, RequesterExt};
use tokio::{sync::Notify, time::timeout};

pub struct Dispatcher<R, Err> {
    requester: R,
    cache_me_requester: CacheMe<R>,
    dependencies: DependencyMap,

    handler: UpdateHandler<Err>,
    default_handler: DefaultHandler,
    error_handler: Arc<dyn ErrorHandler<Err>>,
    allowed_updates: HashSet<AllowedUpdate>,

    state: Arc<DispatcherState>,
    shutdown_notify_back: Arc<Notify>,
}

// TODO: it is allowed to return message as response on telegram request in
// webhooks, so we can allow this too. See more there: https://core.telegram.org/bots/api#making-requests-when-getting-updates
// FIXME: remove 'static lifetime?
pub type UpdateHandler<Err> = dptree::Handler<'static, DependencyMap, Result<(), Err>>;
pub type DefaultHandler = dptree::Handler<'static, DependencyMap, (), Infallible>;

impl<R, Err> Dispatcher<R, Err>
where
    R: Requester + Clone + Send + Sync + 'static,
    Err: Send + Sync + 'static,
{
    pub fn new(requester: R, handler: UpdateHandler<Err>) -> Self
    where
        Err: Debug,
    {
        Dispatcher {
            requester: requester.clone(),
            cache_me_requester: requester.cache_me(),
            dependencies: DependencyMap::new(),
            handler,
            default_handler: dptree::endpoint(|update: Update| async move {
                log::warn!("Unhandled update: {:?}", update)
            }),
            error_handler: LoggingErrorHandler::new(),
            allowed_updates: Default::default(),
            state: Arc::new(Default::default()),
            shutdown_notify_back: Arc::new(Default::default()),
        }
    }

    /// Setup the `^C` handler which [`shutdown`]s dispatching.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    #[cfg(feature = "ctrlc_handler")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ctrlc_handler")))]
    #[must_use = "Call .dispatch() or .dispatch_with_listener() function to start dispatching."]
    pub fn setup_ctrlc_handler(self) -> Self {
        let state = Arc::clone(&self.state);
        tokio::spawn(async move {
            loop {
                tokio::signal::ctrl_c().await.expect("Failed to listen for ^C");

                match shutdown_inner(&state) {
                    Ok(()) => log::info!("^C received, trying to shutdown the dispatcher..."),
                    Err(Ok(_)) => {
                        log::info!(
                            "^C received, the dispatcher is already shutting down, ignoring the \
                             signal"
                        )
                    }
                    Err(Err(_)) => {
                        log::info!("^C received, the dispatcher isn't running, ignoring the signal")
                    }
                }
            }
        });

        self
    }

    /// Returns a shutdown token, which can later be used to shutdown
    /// dispatching.
    pub fn shutdown_token(&self) -> ShutdownToken {
        ShutdownToken {
            dispatcher_state: Arc::clone(&self.state),
            shutdown_notify_back: Arc::clone(&self.shutdown_notify_back),
        }
    }

    /// Starts your bot with the default parameters.
    ///
    /// The default parameters are a long polling update listener and log all
    /// errors produced by this listener).
    ///
    /// Please note that after shutting down (either because of [`shutdown`],
    /// [a ctrlc signal], or [`UpdateListener`] returning `None`) all handlers
    /// will be gone. As such, to restart listening you need to re-add
    /// handlers.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    /// [a ctrlc signal]: Dispatcher::setup_ctrlc_handler
    pub async fn dispatch(&mut self)
    where
        R: Requester + Clone,
        <R as Requester>::GetUpdatesFaultTolerant: Send,
    {
        let listener = update_listeners::polling_default(self.requester.clone()).await;
        let error_handler =
            LoggingErrorHandler::with_custom_text("An error from the update listener");

        self.dispatch_with_listener(listener, error_handler).await;
    }

    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
    ///
    /// Please note that after shutting down (either because of [`shutdown`],
    /// [a ctrlc signal], or [`UpdateListener`] returning `None`) all handlers
    /// will be gone. As such, to restart listening you need to re-add
    /// handlers.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    /// [a ctrlc signal]: Dispatcher::setup_ctrlc_handler
    pub async fn dispatch_with_listener<'a, UListener, ListenerE, Eh>(
        &'a mut self,
        mut update_listener: UListener,
        update_listener_error_handler: Arc<Eh>,
    ) where
        UListener: UpdateListener<ListenerE> + 'a,
        Eh: ErrorHandler<ListenerE> + 'a,
        ListenerE: Debug,
    {
        use crate::dispatching::ShutdownState::*;

        update_listener.hint_allowed_updates(&mut self.allowed_updates.clone().into_iter());

        let shutdown_check_timeout = shutdown_check_timeout_for(&update_listener);
        let mut stop_token = Some(update_listener.stop_token());

        if let Err(actual) = self.state.compare_exchange(Idle, Running) {
            unreachable!(
                "Dispatching is already running: expected `{:?}` state, found `{:?}`",
                Idle, actual
            );
        }

        {
            let stream = update_listener.as_stream();
            tokio::pin!(stream);

            loop {
                // False positive
                #[allow(clippy::collapsible_match)]
                if let Ok(upd) = timeout(shutdown_check_timeout, stream.next()).await {
                    match upd {
                        None => break,
                        Some(upd) => self.process_update(upd, &update_listener_error_handler).await,
                    }
                }

                if let ShuttingDown = self.state.load() {
                    if let Some(token) = stop_token.take() {
                        log::debug!("Start shutting down dispatching...");
                        token.stop();
                        break;
                    }
                }
            }
        }

        if let ShuttingDown = self.state.load() {
            // Stopped because of a `shutdown` call.

            // Notify `shutdown`s that we finished
            self.shutdown_notify_back.notify_waiters();
            log::info!("Dispatching has been shut down.");
        } else {
            log::info!("Dispatching has been stopped (listener returned `None`).");
        }

        self.state.store(Idle);
    }

    async fn process_update<LErr, LErrHandler>(
        &self,
        update: Result<Update, LErr>,
        err_handler: &Arc<LErrHandler>,
    ) where
        LErrHandler: ErrorHandler<LErr>,
    {
        match update {
            Ok(upd) => {
                let mut deps = self.dependencies.clone();
                deps.insert(upd);
                deps.insert(self.requester.clone());
                deps.insert(
                    self.cache_me_requester.get_me().send().await.expect("Failed to retrieve 'me'"),
                );

                match self.handler.dispatch(deps).await {
                    ControlFlow::Break(Ok(())) => {}
                    ControlFlow::Break(Err(err)) => {
                        self.error_handler.clone().handle_error(err).await
                    }
                    ControlFlow::Continue(deps) => {
                        match self.default_handler.clone().dispatch(deps).await {
                            ControlFlow::Break(()) => {}
                            ControlFlow::Continue(_) => unreachable!(
                                "This is unreachable due to Infallible type in the DefaultHandler \
                                 type"
                            ),
                        }
                    }
                }
            }
            Err(err) => err_handler.clone().handle_error(err).await,
        }
    }

    #[must_use = "Call .dispatch() or .dispatch_with_listener() function to start dispatching."]
    // Specify handler that will be called if other handlers was not handle the
    // update.
    pub fn default_handler(self, handler: DefaultHandler) -> Self {
        Dispatcher { default_handler: handler, ..self }
    }

    #[must_use = "Call .dispatch() or .dispatch_with_listener() function to start dispatching."]
    pub fn error_handler(self, handler: Arc<dyn ErrorHandler<Err>>) -> Self {
        Dispatcher { error_handler: handler, ..self }
    }

    #[must_use = "Call .dispatch() or .dispatch_with_listener() function to start dispatching."]
    // Specify dependencies that can be used inside of handlers.
    pub fn dependencies(self, dependencies: DependencyMap) -> Self {
        Dispatcher { dependencies, ..self }
    }
}
