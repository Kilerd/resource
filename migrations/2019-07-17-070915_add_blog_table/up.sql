-- Your SQL goes here
CREATE TABLE "public"."blog"
(
    "id"                       serial,
    "title"                    text        NOT NULL,
    "link"                     text        NOT NULL,
    "last_update_at"           timestamptz NOT NULL DEFAULT NOW(),
    "last_rust_post_update_at" timestamptz NOT NULL DEFAULT NOW(),
    "create_at"                timestamptz NOT NULL DEFAULT NOW(),
    "rss"                      text,
    PRIMARY KEY ("id")
);