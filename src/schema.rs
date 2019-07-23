table! {
    blog (id) {
        id -> Int4,
        title -> Nullable<Text>,
        link -> Text,
        description -> Nullable<Text>,
        rss -> Nullable<Text>,
        confirmed -> Bool,
        last_update_at -> Timestamptz,
        last_rust_post_update_at -> Timestamptz,
        create_at -> Timestamptz,
    }
}
