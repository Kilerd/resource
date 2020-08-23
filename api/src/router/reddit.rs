use std::default::Default;

use actix_web::{get, web, Responder};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use telegram_typing_bot::method::send::SendMessage;
use telegram_typing_bot::typing::ReplyMarkup;
use telegram_typing_bot::typing::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
use tera::Context;
use tokio::time::{Duration, Instant};

use crate::data::AppData;
use crate::model::reddit::Reddit;
use crate::router::AppResponder;
use crate::TELEGRAM_RESOURCE_CHANNEL;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub kind: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub modhash: String,
    pub dist: i64,
    pub children: Vec<Children>,
    pub after: String,
    pub before: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub kind: String,
    pub data: Data2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data2 {
    #[serde(rename = "approved_at_utc")]
    pub approved_at_utc: ::serde_json::Value,
    pub subreddit: String,
    pub selftext: String,
    #[serde(rename = "author_fullname")]
    pub author_fullname: String,
    pub saved: bool,
    #[serde(rename = "mod_reason_title")]
    pub mod_reason_title: ::serde_json::Value,
    pub gilded: i64,
    pub clicked: bool,
    pub title: String,
    #[serde(rename = "link_flair_richtext")]
    pub link_flair_richtext: Vec<::serde_json::Value>,
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit_name_prefixed: String,
    pub hidden: bool,
    pub pwls: i64,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<String>,
    pub downs: i64,
    #[serde(rename = "hide_score")]
    pub hide_score: bool,
    pub name: String,
    pub quarantine: bool,
    #[serde(rename = "link_flair_text_color")]
    pub link_flair_text_color: String,
    #[serde(rename = "author_flair_background_color")]
    pub author_flair_background_color: Option<String>,
    #[serde(rename = "subreddit_type")]
    pub subreddit_type: String,
    pub ups: i64,
    #[serde(rename = "total_awards_received")]
    pub total_awards_received: i64,
    #[serde(rename = "media_embed")]
    pub media_embed: MediaEmbed,
    #[serde(rename = "author_flair_template_id")]
    pub author_flair_template_id: ::serde_json::Value,
    #[serde(rename = "is_original_content")]
    pub is_original_content: bool,
    #[serde(rename = "user_reports")]
    pub user_reports: Vec<::serde_json::Value>,
    #[serde(rename = "secure_media")]
    pub secure_media: ::serde_json::Value,
    #[serde(rename = "is_reddit_media_domain")]
    pub is_reddit_media_domain: bool,
    #[serde(rename = "is_meta")]
    pub is_meta: bool,
    pub category: ::serde_json::Value,
    #[serde(rename = "secure_media_embed")]
    pub secure_media_embed: SecureMediaEmbed,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<String>,
    #[serde(rename = "can_mod_post")]
    pub can_mod_post: bool,
    pub score: i32,
    #[serde(rename = "approved_by")]
    pub approved_by: ::serde_json::Value,
    #[serde(rename = "author_premium")]
    pub author_premium: bool,
    pub thumbnail: String,
    pub edited: ::serde_json::Value,
    #[serde(rename = "author_flair_css_class")]
    pub author_flair_css_class: Option<String>,
    #[serde(rename = "author_flair_richtext")]
    pub author_flair_richtext: Vec<::serde_json::Value>,
    pub gildings: Gildings,
    #[serde(rename = "content_categories")]
    pub content_categories: ::serde_json::Value,
    #[serde(rename = "is_self")]
    pub is_self: bool,
    #[serde(rename = "mod_note")]
    pub mod_note: ::serde_json::Value,
    pub created: f64,
    #[serde(rename = "link_flair_type")]
    pub link_flair_type: String,
    pub wls: i64,
    #[serde(rename = "removed_by_category")]
    pub removed_by_category: ::serde_json::Value,
    #[serde(rename = "banned_by")]
    pub banned_by: ::serde_json::Value,
    #[serde(rename = "author_flair_type")]
    pub author_flair_type: String,
    pub domain: String,
    #[serde(rename = "allow_live_comments")]
    pub allow_live_comments: bool,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    pub likes: ::serde_json::Value,
    #[serde(rename = "suggested_sort")]
    pub suggested_sort: Option<String>,
    #[serde(rename = "banned_at_utc")]
    pub banned_at_utc: ::serde_json::Value,
    #[serde(rename = "view_count")]
    pub view_count: ::serde_json::Value,
    pub archived: bool,
    #[serde(rename = "no_follow")]
    pub no_follow: bool,
    #[serde(rename = "is_crosspostable")]
    pub is_crosspostable: bool,
    pub pinned: bool,
    #[serde(rename = "over_18")]
    pub over18: bool,
    #[serde(rename = "all_awardings")]
    pub all_awardings: Vec<AllAwarding>,
    pub awarders: Vec<::serde_json::Value>,
    #[serde(rename = "media_only")]
    pub media_only: bool,
    #[serde(rename = "can_gild")]
    pub can_gild: bool,
    pub spoiler: bool,
    pub locked: bool,
    #[serde(rename = "author_flair_text")]
    pub author_flair_text: Option<String>,
    pub visited: bool,
    #[serde(rename = "removed_by")]
    pub removed_by: ::serde_json::Value,
    #[serde(rename = "num_reports")]
    pub num_reports: ::serde_json::Value,
    pub distinguished: Option<String>,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: String,
    #[serde(rename = "mod_reason_by")]
    pub mod_reason_by: ::serde_json::Value,
    #[serde(rename = "removal_reason")]
    pub removal_reason: ::serde_json::Value,
    #[serde(rename = "link_flair_background_color")]
    pub link_flair_background_color: String,
    pub id: String,
    #[serde(rename = "is_robot_indexable")]
    pub is_robot_indexable: bool,
    #[serde(rename = "report_reasons")]
    pub report_reasons: ::serde_json::Value,
    pub author: String,
    #[serde(rename = "discussion_type")]
    pub discussion_type: ::serde_json::Value,
    #[serde(rename = "num_comments")]
    pub num_comments: i64,
    #[serde(rename = "send_replies")]
    pub send_replies: bool,
    #[serde(rename = "whitelist_status")]
    pub whitelist_status: String,
    #[serde(rename = "contest_mode")]
    pub contest_mode: bool,
    #[serde(rename = "mod_reports")]
    pub mod_reports: Vec<::serde_json::Value>,
    #[serde(rename = "author_patreon_flair")]
    pub author_patreon_flair: bool,
    #[serde(rename = "author_flair_text_color")]
    pub author_flair_text_color: Option<String>,
    pub permalink: String,
    #[serde(rename = "parent_whitelist_status")]
    pub parent_whitelist_status: String,
    pub stickied: bool,
    pub url: String,
    #[serde(rename = "subreddit_subscribers")]
    pub subreddit_subscribers: i64,
    #[serde(rename = "created_utc")]
    pub created_utc: f64,
    #[serde(rename = "num_crossposts")]
    pub num_crossposts: i64,
    pub media: ::serde_json::Value,
    #[serde(rename = "is_video")]
    pub is_video: bool,
    #[serde(rename = "link_flair_template_id")]
    pub link_flair_template_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecureMediaEmbed {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Gildings {
    #[serde(rename = "gid_2")]
    pub gid2: Option<i64>,
    #[serde(rename = "gid_1")]
    pub gid1: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllAwarding {
    pub count: i64,
    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: ::serde_json::Value,
    pub description: String,
    #[serde(rename = "end_date")]
    pub end_date: ::serde_json::Value,
    #[serde(rename = "coin_reward")]
    pub coin_reward: i64,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "days_of_premium")]
    pub days_of_premium: i64,
    #[serde(rename = "coin_price")]
    pub coin_price: i64,
    #[serde(rename = "is_new")]
    pub is_new: bool,
    #[serde(rename = "icon_format")]
    pub icon_format: ::serde_json::Value,
    #[serde(rename = "award_sub_type")]
    pub award_sub_type: String,
    #[serde(rename = "resized_icons")]
    pub resized_icons: Vec<ResizedIcon>,
    #[serde(rename = "icon_height")]
    pub icon_height: i64,
    #[serde(rename = "award_type")]
    pub award_type: String,
    #[serde(rename = "start_date")]
    pub start_date: ::serde_json::Value,
    #[serde(rename = "days_of_drip_extension")]
    pub days_of_drip_extension: i64,
    pub id: String,
    #[serde(rename = "icon_width")]
    pub icon_width: i64,
    #[serde(rename = "subreddit_coin_reward")]
    pub subreddit_coin_reward: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResizedIcon {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

//#[cfg(debug_assertions)]
//pub async fn a() -> Result<Root, ()> {
//    let result = std::fs::read_to_string("reddit.json");
//    Ok(serde_json::from_str::<Root>(&result.unwrap()).unwrap())
//}

pub async fn a() -> Result<Root, ()> {
    let result = surf::get("https://www.reddit.com/r/rust/.json")
        .set_header("User-Agent", "rust:resource:v0.1.0 (by /u/kilerd_chan)")
        .await;
    match result {
        Ok(mut res) => {
            let body = res.body_string().await;
            match body {
                Ok(body_string) => {
                    let root_result = serde_json::from_str::<Root>(&body_string);
                    match root_result {
                        Ok(root) => {
                            return Ok(root);
                        }
                        Err(e) => {
                            error!(
                                "cannot serde reddit response as Root: {} with response: {}",
                                e, body_string
                            );
                        }
                    }
                }
                Err(e) => {
                    error!("fetching reddig json error: {}", e);
                }
            }
        }
        Err(e) => {
            error!("cannot fetch reddit rending: {}", e);
        }
    }
    Err(())
}

async fn sending_topic_to_telegram_channel(data: AppData, topic: Reddit) -> () {
    if topic.telegram_message_id.is_none() {
        info!(
            "sending new reddit topic to telegram channel: {}",
            &topic.title
        );
        let mut message_payload = format!("\\[Reddit] *{}*\n", &topic.title);
        if let Some(selftext) = &topic.selftext {
            let mut lines = selftext.lines().peekable();
            while lines.peek().is_some() && message_payload.len() < 350 {
                if let Some(content) = lines.next() {
                    message_payload.push_str(&format!("\n{}", content));
                }
            }
        }
        let reddit_link = format!("https://www.reddit.com{}", &topic.permalink);
        if topic.url.ne(&reddit_link) {
            message_payload.push_str(&format!("\n\n{}", &topic.url));
        }
        let mut message = SendMessage::new(TELEGRAM_RESOURCE_CHANNEL.clone(), message_payload)
            .disable_web_page_preview(false)
            .parse_mode(ParseMode::Markdown);

        let mut vec1 = vec![];

        vec1.push(InlineKeyboardButton {
            text: "Reddit Comment".to_string(),
            url: Some(reddit_link),
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        });

        message.reply_markup = Some(ReplyMarkup::InlineKeyboardMarkup(InlineKeyboardMarkup {
            inline_keyboard: vec![vec1],
        }));
        let result1 = data.bot.request(message).await;
        match result1 {
            Ok(callback_message) => {
                diesel::update(crate::schema::reddits::table)
                    .filter(crate::schema::reddits::id.eq(&topic.id))
                    .set(
                        crate::schema::reddits::telegram_message_id
                            .eq(format!("{}", callback_message.message_id)),
                    )
                    .execute(&data.postgres());
            }
            Err(e) => {
                error!(
                    "error on sending redit topic to telegram channel: {}",
                    e.description
                );
            }
        }
    }
}

pub async fn looping_fetch(data: AppData) {
    let mut interval = tokio::time::interval_at(
        Instant::now() + Duration::from_secs(60),
        Duration::from_secs(60 * 10),
    );
    loop {
        interval.tick().await;
        let root = a().await;
        if let Ok(root) = root {
            root.data
                .children
                .into_iter()
                .rev()
                .map(|c| {
                    (
                        c.data.id,
                        c.data.score,
                        c.data.title,
                        c.data.selftext,
                        c.data.author,
                        c.data.permalink,
                        c.data.url,
                    )
                })
                .filter(|c| c.1 >= 50)
                .for_each(|(id, score, title, selftext, author, permalink, url)| {
                    let reddit = Reddit {
                        id,
                        score,
                        title,
                        selftext: if selftext.eq("") {
                            None
                        } else {
                            Some(selftext)
                        },
                        author,
                        permalink,
                        url,
                        create_time: Utc::now(),
                        telegram_message_id: None,
                    };
                    let result = diesel::insert_into(crate::schema::reddits::table)
                        .values(&reddit)
                        .on_conflict(crate::schema::reddits::id)
                        .do_update()
                        .set(crate::schema::reddits::score.eq(score))
                        .get_result::<Reddit>(&data.postgres());
                    if let Ok(topic) = result {
                        tokio::spawn(sending_topic_to_telegram_channel(data.clone(), topic));
                    }
                });
        }
    }
}

#[get("/trending")]
pub async fn reddit_rending_api(data: web::Data<AppData>) -> impl Responder {
    let x = crate::schema::reddits::table
        .order(crate::schema::reddits::create_time.desc())
        .limit(50)
        .load::<Reddit>(&data.postgres())
        .expect("cannot load reddit rending");

    AppResponder::json(x)
}
