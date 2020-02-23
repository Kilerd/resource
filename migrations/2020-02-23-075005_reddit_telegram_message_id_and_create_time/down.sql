-- This file should undo anything in `up.sql`
ALTER TABLE "public"."reddits" DROP COLUMN "create_time";
ALTER TABLE "public"."reddits" DROP COLUMN "telegram_message_id";
