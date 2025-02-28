//! Generated by `codegen_payloads`, do not edit by hand.

use serde::Serialize;

use crate::types::LabeledPrice;

impl_payload! {
    /// Use this method to create a link for an invoice. Returns the created invoice link as String on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub CreateInvoiceLink (CreateInvoiceLinkSetters) => String {
        required {
            /// Product name, 1-32 characters
            pub title: String [into],
            /// Product description, 1-255 characters
            pub description: String [into],
            /// Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use for your internal processes.
            pub payload: String [into],
            /// Payments provider token, obtained via [Botfather]. Pass an empty string for payments in [Telegram Stars].
            ///
            /// [Botfather]: https://t.me/botfather
            /// [Telegram Stars]: https://t.me/BotNews/90
            pub provider_token: String [into],
            /// Three-letter ISO 4217 currency code, see [more on currencies]. Pass `XTR` for payments in [Telegram Stars].
            ///
            /// [more on currencies]: https://core.telegram.org/bots/payments#supported-currencies
            /// [Telegram Stars]: https://t.me/BotNews/90
            pub currency: String [into],
            /// Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.)
            pub prices: Vec<LabeledPrice> [collect],
        }
        optional {
            /// The maximum accepted amount for tips in the smallest units of the currency (integer, **not** float/double). For example, for a maximum tip of `US$ 1.45` pass `max_tip_amount = 145`. See the exp parameter in [`currencies.json`], it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0
            ///
            /// [`currencies.json`]: https://core.telegram.org/bots/payments/currencies.json
            pub max_tip_amount: u32,
            /// A JSON-serialized array of suggested amounts of tips in the smallest units of the currency (integer, **not** float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed _max_tip_amount_.
            pub suggested_tip_amounts: Vec<u32> [collect],
            /// A JSON-serialized data about the invoice, which will be shared with the payment provider. A detailed description of required fields should be provided by the payment provider.
            pub provider_data: String [into],
            /// URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service. People like it better when they see what they are paying for.
            pub photo_url: String [into],
            /// Photo size in bytes
            pub photo_size: String [into],
            /// Photo width
            pub photo_width: String [into],
            /// Photo height
            pub photo_height: String [into],
            /// Pass _True_, if you require the user's full name to complete the order
            pub need_name: bool,
            /// Pass _True_, if you require the user's phone number to complete the order
            pub need_phone_number: bool,
            /// Pass _True_, if you require the user's email address to complete the order
            pub need_email: bool,
            /// Pass _True_, if you require the user's shipping address to complete the order
            pub need_shipping_address: bool,
            /// Pass _True_, if user's phone number should be sent to provider
            pub send_phone_number_to_provider: bool,
            /// Pass _True_, if user's email address should be sent to provider
            pub send_email_to_provider: bool,
            /// Pass _True_, if the final price depends on the shipping method
            pub is_flexible: bool,
        }
    }
}
