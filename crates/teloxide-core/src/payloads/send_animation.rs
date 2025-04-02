//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::{
    BusinessConnectionId, InputFile, Message, MessageEntity, ParseMode, Recipient, ReplyMarkup,
    ReplyParameters, ThreadId,
};

impl_payload! {
    @[multipart = animation, thumbnail]
    /// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent [`Message`] is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
    ///
    /// [`Message`]: crate::types::Message
    #[derive(Debug, Clone, Serialize)]
    pub SendAnimation (SendAnimationSetters) => Message {
        required {
            /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
            pub chat_id: Recipient [into],
            /// Animation to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub animation: InputFile,
        }
        optional {
            /// Unique identifier of the business connection on behalf of which the message will be sent
            pub business_connection_id: BusinessConnectionId,
            /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
            pub message_thread_id: ThreadId,
            /// Duration of the animation in seconds
            pub duration: u32,
            /// Animation width
            pub width: u32,
            /// Animation height
            pub height: u32,
            /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. [More info on Sending Files »]
            ///
            /// [More info on Sending Files »]: crate::types::InputFile
            pub thumbnail: InputFile,
            /// Animation caption (may also be used when resending videos by _file\_id_), 0-1024 characters after entities parsing
            pub caption: String [into],
            /// Mode for parsing entities in the animation caption. See [formatting options] for more details.
            ///
            /// [formatting options]: https://core.telegram.org/bots/api#formatting-options
            pub parse_mode: ParseMode,
            /// List of special entities that appear in the photo caption, which can be specified instead of _parse\_mode_
            pub caption_entities: Vec<MessageEntity> [collect],
            /// Pass True, if the caption must be shown above the message media
            pub show_caption_above_media: bool,
            /// Pass True if the animation needs to be covered with a spoiler animation
            pub has_spoiler: bool,
            /// Sends the message [silently]. Users will receive a notification with no sound.
            ///
            /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
            pub disable_notification: bool,
            /// Protects the contents of sent messages from forwarding and saving
            pub protect_content: bool,
            /// Unique identifier of the message effect to be added to the message; for private chats only
            pub message_effect_id: String [into],
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
