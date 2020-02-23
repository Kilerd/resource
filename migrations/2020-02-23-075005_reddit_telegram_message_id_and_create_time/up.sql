-- Your SQL goes here
ALTER TABLE "public"."reddits" ADD COLUMN "create_time" timestamptz NOT NULL DEFAULT NOW();
ALTER TABLE "public"."reddits" ADD COLUMN "telegram_message_id" text