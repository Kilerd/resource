use actix_web::{get, web, Responder};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use telegram_typing_bot::method::send::SendMessage;
use telegram_typing_bot::typing::ReplyMarkup;
use telegram_typing_bot::typing::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};

use tokio::time::{Duration, Instant};

use crate::data::AppData;
use crate::model::reddit::Reddit;
use crate::router::AppResponder;
use crate::TELEGRAM_RESOURCE_CHANNEL;

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub children: Vec<Children>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Children {
    pub data: Data2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data2 {
    pub selftext: String,
    pub title: String,
    pub score: i32,
    pub author: String,
    pub id: String,
    pub permalink: String,
    pub url: String,
}

#[cfg(debug_assertions)]
pub async fn a() -> Result<Root, ()> {
    let result = std::fs::read_to_string("reddit.json");
    Ok(serde_json::from_str::<Root>(&result.unwrap()).unwrap())
}

#[cfg(not(debug_assertions))]
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
    if topic.telegram_message_id.is_some() {
        return ();
    }

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
            let update_result = diesel::update(crate::schema::reddits::table)
                .filter(crate::schema::reddits::id.eq(&topic.id))
                .set(
                    crate::schema::reddits::telegram_message_id
                        .eq(format!("{}", callback_message.message_id)),
                )
                .execute(&data.postgres());
            if let Err(e) = update_result {
                error!("error on updating telegram_id: {}", e);
            }
        }
        Err(e) => {
            error!(
                "error on sending redit topic to telegram channel: {}",
                e.description
            );
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
        dbg!(&root);
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
