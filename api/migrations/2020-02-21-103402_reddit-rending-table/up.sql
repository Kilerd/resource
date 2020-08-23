-- Your SQL goes here
CREATE TABLE "public"."reddits"
(
    "id"        text NOT NULL,
    "score"     int4 NOT NULL DEFAULT 0,
    "title"     text NOT NULL,
    "selftext"  text,
    "author"    text NOT NULL,
    "permalink" text NOT NULL,
    "url"       text NOT NULL,
    PRIMARY KEY ("id")
);