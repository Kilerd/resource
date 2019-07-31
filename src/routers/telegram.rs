use crate::{
    data::AppData,
    models::post::{sql::NewPost, Post},
    TELEGRAM_WHITE_LSIT,
};
use actix_web::{
    client::Client,
    post,
    test::block_on,
    web::{block, Data, Json},
    Responder,
};
use futures_util::future::FutureExt;
use regex::Regex;
use telegram_typing_bot::{
    bot::Bot,
    error::ApiResult,
    methods::{basic::SendMessage, update::Update},
    typing::{ParseMode, User},
};

#[post("")]
pub fn telegram_web_hook(update: Json<Update>, data: Data<AppData>) -> impl Responder {
    //    dbg!(update);
    if let Some(message) = &update.message {
        let (message_user_id, creator) = message
            .from
            .as_ref()
            .map(|user| (user.id, user.first_name.clone()))
            .unwrap_or((-1, String::new()));

        if TELEGRAM_WHITE_LSIT.contains(&message_user_id) {
            let text = message.text.clone().unwrap_or_default();
            if text.starts_with(r"/post") {
                info!(
                    "adding post data from {} with text {}",
                    message_user_id, text
                );
                let re = Regex::new(r"\[(.+)\]\((.+)\)(\{(.*)\})?").unwrap();
                for cap in re.captures_iter(text.as_ref()) {
                    //                dbg!(cap);
                    let title = &cap[1];
                    let link = &cap[2];
                    let description = cap.get(4).map(|m| m.as_str().to_string());
                    //                let description = String::from_utf8_lossy(&cap[4]);
                    info!("adding post title: {} link: {}", title, link);
                    let new_post = NewPost {
                        title: title.to_string(),
                        link: link.to_string(),
                        description,
                        from_blog: None,
                        creator: creator.clone(),
                    };
                    let option = Post::insert(new_post, &data.postgres());
                    let msg = if let Some(post) = option {
                        format!("Successfully added as Post {}", post.id)
                    } else {
                        String::from("Fail to add")
                    };
                    let send_message_payload = SendMessage {
                        chat_id: message.chat.id.to_string(),
                        text: msg,
                        parse_mod: None,
                        disable_web_page_preview: None,
                        disable_notification: None,
                        reply_to_message_id: Some(message.message_id),
                        reply_markup: None,
                    };
                    data.bot.do_send(send_message_payload);
                }
            };
        } else {
            let send_message_payload = SendMessage {
                chat_id: message.chat.id.to_string(),
                text: format!("you are not in the white list"),
                parse_mod: None,
                disable_web_page_preview: None,
                disable_notification: None,
                reply_to_message_id: Some(message.message_id),
                reply_markup: None,
            };
            data.bot.do_send(send_message_payload);
        };
    };
    "True"
}
