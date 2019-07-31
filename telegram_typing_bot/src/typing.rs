use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: i32,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    pub username: Option<String>,
    /// [IETF language tag](https://en.wikipedia.org/wiki/IETF_language_tag) of the user's language
    pub language_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum ChatType {
    Private,
    Group,
    SuperGroup,
    Channel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chat {
    /// Unique identifier for this chat.
    pub id: i64,
    /// Type of chat
    #[serde(rename = "type")]
    pub chat_type: ChatType,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
    /// Chat photo. Returned only in getChat.
    pub photo: Option<ChatPhoto>,
    /// Description, for supergroups and channel chats. Returned only in getChat.
    pub description: Option<String>,
    /// Chat invite link, for supergroups and channel chats. Each administrator in a chat generates their own invite links, so the bot must first generate the link using exportChatInviteLink. Returned only in getChat.
    pub invite_link: Option<String>,
    /// Pinned message, for groups, supergroups and channels. Returned only in getChat.
    pub pinned_message: Option<Box<Message>>,
    /// For supergroups, name of group sticker set. Returned only in getChat.
    pub sticker_set_name: Option<String>,
    /// True, if the bot can change the group sticker set. Returned only in getChat.
    pub can_set_sticker_set: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_id: i32,
    pub from: Option<User>,
    pub date: i32,
    pub chat: Box<Chat>,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Box<Chat>>,
    pub forward_from_message_id: Option<i32>,
    pub forward_signature: Option<String>,
    pub forward_sender_name: Option<String>,
    pub forward_date: Option<i32>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<i32>,
    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub caption_entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
    pub animation: Option<Animation>,
    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub video_note: Option<VideoNote>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub poll: Option<Poll>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,
    pub migrate_to_chat_id: Option<bool>,
    pub migrate_from_chat_id: Option<i64>,
    pub pinned_message: Option<Box<Message>>,
    pub invoice: Option<Invoice>,
    pub successful_payment: Option<SuccessfulPayment>,
    pub connected_website: Option<String>,
    pub passport_data: Option<PassportData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageEntity {
    #[serde(rename = "type")]
    pub message_type: String,
    pub offset: i32,
    pub length: i32,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Audio {
    pub file_id: String,
    pub duration: i32,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
    pub thumb: Option<PhotoSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Video {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Animation {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Voice {
    file_id: String,
    duration: i32,
    mime_type: Option<String>,
    file_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VideoNote {
    file_id: String,
    length: i32,
    duration: i32,
    thumb: Option<PhotoSize>,
    file_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contact {
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    user_id: Option<i32>,
    vcard: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Location {
    longitude: f64,
    latitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Venue {
    location: Location,
    title: String,
    address: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollOption {
    text: String,
    voter_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll {
    id: String,
    question: String,
    options: Vec<PollOption>,
    is_closed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserProfilePhotos {
    total_count: i32,
    photo: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    file_id: String,
    file_size: Option<i32>,
    file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    resize_keyboard: Option<bool>,
    one_time_keyboard: Option<bool>,
    selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyboardButton {
    text: String,
    request_contact: Option<bool>,
    request_location: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReplyKeyboardRemove {
    /// should be true
    remove_keyboard: bool,
    selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InlineKeyboardButton {
    text: String,
    url: Option<String>,
    callback_data: Option<String>,
    switch_inline_query: Option<String>,
    switch_inline_query_current_chat: Option<String>,
    callback_game: Option<CallbackGame>,
    pay: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallbackQuery {
    id: String,
    from: User,
    message: Option<Message>,
    inline_message_id: Option<String>,
    chat_instance: String,
    data: Option<String>,
    game_short_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForceReply {
    /// should be true
    force_reply: bool,
    selective: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Restricted,
    Left,
    Kicked,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMember {
    user: User,
    status: ChatMemberStatus,
    until_date: Option<i32>,
    can_be_edited: Option<bool>,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
    is_member: Option<bool>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_message: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i32>,
    retry_after: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParseMode {
    Markdown,
    Html,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMediaPhoto {
    /// always be photo
    #[serde(rename = "type")]
    photo_type: String,
    media: String,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMediaVideo {
    #[serde(rename = "type")]
    video_type: String,
    media: String,
    /// TODO inputFile
    thumb: Option<String>,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    width: Option<i32>,
    height: Option<i32>,
    duration: Option<i32>,
    supports_streaming: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMediaAudio {
    #[serde(rename = "type")]
    audio_type: String,
    media: String,
    thumb: Option<String>,
    caption: Option<String>,
    parse_mod: Option<ParseMode>,
    duration: Option<i32>,
    performer: Option<String>,
    title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMediaAnimation {
    #[serde(rename = "type")]
    animation_type: String,
    media: String,
    thumb: Option<String>,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    width: Option<i32>,
    height: Option<i32>,
    duration: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InputMediaDocument {
    #[serde(rename = "type")]
    document_type: String,
    media: String,
    thumb: Option<String>,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sticker {
    file_id: String,
    width: i32,
    height: i32,
    thumb: Option<PhotoSize>,
    emoji: Option<String>,
    set_name: Option<String>,
    mask_position: Option<String>,
    fil_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Game {
    title: String,
    description: String,
    photo: Vec<PhotoSize>,
    text: Option<String>,
    text_entities: Option<Vec<MessageEntity>>,
    animation: Option<Animation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallbackGame;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invoice {
    title: String,
    description: String,
    start_parameter: String,
    currency: String,
    total_amount: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuccessfulPayment {
    currency: String,
    total_amount: i32,
    invoice_payload: String,
    shipping_option_id: Option<String>,
    order_info: Option<OrderInfo>,
    telegram_payment_charge_id: String,
    provider_payment_charge_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderInfo {
    name: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShippingAddress {
    country_code: String,
    state: String,
    city: String,
    street_line1: String,
    street_line2: String,
    post_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PassportData;
