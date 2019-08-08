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
        initialized -> Bool,
    }
}

table! {
    post (id) {
        id -> Int4,
        title -> Text,
        link -> Text,
        description -> Nullable<Text>,
        create_at -> Timestamptz,
        from_blog -> Nullable<Int4>,
        creator -> Text,
    }
}

joinable!(post -> blog (from_blog));

allow_tables_to_appear_in_same_query!(blog, post,);
