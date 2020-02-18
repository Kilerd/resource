-- Your SQL goes here
CREATE TABLE "public"."blog"
(
    "id"                       serial,
    "title"                    text,
    "link"                     text        NOT NULL,
    "description"              text,
    "rss"                      text,
    "confirmed"                boolean     NOT NULL DEFAULT FALSE,
    "last_update_at"           timestamptz NOT NULL DEFAULT NOW(),
    "last_rust_post_update_at" timestamptz NOT NULL DEFAULT NOW(),
    "create_at"                timestamptz NOT NULL DEFAULT NOW(),
    PRIMARY KEY ("id")
);