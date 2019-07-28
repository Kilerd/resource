-- Your SQL goes here
CREATE TABLE "public"."post"
(
    "id"          serial,
    "title"       text        NOT NULL,
    "link"        text        NOT NULL,
    "description" text,
    "create_at"   timestamptz NOT NULL DEFAULT NOW(),
    "from_blog"   integer,
    "creator"     text        NOT NULL,
    PRIMARY KEY ("id"),
    FOREIGN KEY ("from_blog") REFERENCES "public"."blog" ("id")
);
