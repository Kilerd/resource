table! {
    blog (id) {
        id -> Int4,
        title -> Text,
        link -> Text,
        last_update_at -> Timestamptz,
        last_rust_post_update_at -> Timestamptz,
        create_at -> Timestamptz,
        rss -> Nullable<Text>,
    }
}
