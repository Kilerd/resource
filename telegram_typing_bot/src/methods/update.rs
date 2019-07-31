use crate::{
    bot::Bot,
    error::ApiResult,
    typing::{CallbackQuery, Location, Message, OrderInfo, Poll, ShippingAddress, User},
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
//
//impl_method!(GetUpdates -> Vec<Update>);
//impl_method!(SetWebhook -> ());
//impl_method!(GetWebhookInfo -> WebhookInfo);

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChosenInlineResult {
    pub result_id: String,
    pub from: User,
    pub location: Option<Location>,
    pub inline_message_id: Option<String>,
    pub query: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingQuery {
    pub id: String,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PreCheckoutQuery {
    pub id: String,
    pub from: User,
    pub currency: String,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub shipping_option_id: Option<String>,
    pub order_info: Option<OrderInfo>,
}

/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Update {
    /// The update‘s unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you’re using Webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i32,
    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>,
    /// New incoming inline query
    pub inline_query: Option<Message>,
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// New poll state. Bots receive only updates about polls, which are sent or stopped by the bot
    pub poll: Option<Poll>,
}

/// Contains information about the current status of a webhook.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up
    pub url: String,
    /// True, if a custom certificate was provided for webhook certificate checks
    pub has_custom_certificate: bool,
    /// Number of updates awaiting delivery
    pub pending_update_count: i32,
    /// Unix time for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_date: Option<i32>,
    /// Error message in human-readable format for the most recent error that happened when trying to deliver an update via webhook
    pub last_error_message: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery
    pub max_connections: Option<i32>,
    /// A list of update types the bot is subscribed to. Defaults to all update types
    pub allowed_updates: Option<Vec<String>>,
}
/// The method for receiving incoming updates using long polling
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    pub offset: Option<i32>,
    /// Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    pub limit: Option<i32>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    pub timeout: Option<i32>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    ///
    ///Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to specify a url and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified url, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
///
/// If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    pub certificate: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput.
    pub max_connections: Option<i32>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteWebhook;

/// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetWebhookInfo;
