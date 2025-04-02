//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    MessageEntity, MessageId, ParseMode, Recipient, ReplyMarkup, ReplyParameters, ThreadId,
};

impl_payload! {
    /// Use this method to copy messages of any kind. The method is analogous to the method forwardMessage, but the copied message doesn't have a link to the original message. Returns the [`MessageId`] of the sent message on success.
    ///
    /// [`MessageId`]: crate::types::MessageId
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub CopyMessage (CopyMessageSetters) => MessageId {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Unique identifier for the chat where the original message was sent (or channel username in the format `@channelusername`)
            pub from_chat_id: Recipient [into],
            /// Message identifier in the chat specified in _from\_chat\_id_
            #[serde(flatten)]
            pub message_id: MessageId,
        }
        optional {
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// New caption for media, 0-1024 characters after entities parsing. If not specified, the original caption is kept
            pub caption: String [into],
            /// Mode for parsing entities in the photo caption. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the new caption, which can be specified instead of _parse\_mode_
            pub caption_entities: Vec<MessageEntity> [collect],
            /// Pass True, if the caption must be shown above the message media. Ignored if a new caption isn't specified
            pub show_caption_above_media: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// Description of the message to reply to
            pub reply_parameters: ReplyParameters,
            /// Additional interface options. A JSON-serialized object for an [inline keyboard], [custom reply keyboard], instructions to remove a reply keyboard or to force a reply from the user. Not supported for messages sent on behalf of a business account.
            ///
            /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
            /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
            pub reply_markup: ReplyMarkup [into],
        }
    }
}
