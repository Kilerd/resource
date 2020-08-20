table! {
    reddits (id) {
        id -> Text,
        score -> Int4,
        title -> Text,
        selftext -> Nullable<Text>,
        author -> Text,
        permalink -> Text,
        url -> Text,
        create_time -> Timestamptz,
        telegram_message_id -> Nullable<Text>,
    }
}
