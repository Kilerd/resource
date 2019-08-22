use crate::{
    data::AppData,
    models::{
        blog::{form::NewBlogInput, Blog},
        post::{sql::NewPost, Post},
    },
    TELEGRAM_RELEASE_CHANNEL, TELEGRAM_WHITE_LSIT,
};
use actix_web::{
    post,
    web::{Data, Json},
    Responder,
};
use regex::Regex;
use telegram_typing_bot::method::SendMessage;
use telegram_typing_bot::typing::UpdateMessage;
use telegram_typing_bot::typing::{ParseMode, Update};

#[post("")]
pub fn telegram_web_hook(update: Json<Update>, data: Data<AppData>) -> impl Responder {
    //    dbg!(update);
    if let UpdateMessage::Message(message) = &update.message {
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
                let re = Regex::new(r"\[(.+)\]\(([^)]+)\)(\{([^}]*)\})?").unwrap();
                for cap in re.captures_iter(text.as_ref()) {
                    dbg!(&cap);
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
                        let channel_msg = format!(
                            "[{}]({})\n{}",
                            post.title,
                            post.link,
                            post.description.as_ref().unwrap_or(&"".to_string())
                        );
                        data.bot.do_send(
                            SendMessage::new(TELEGRAM_RELEASE_CHANNEL.as_str(), channel_msg)
                                .parse_mode(ParseMode::Markdown),
                        );

                        format!("Successfully added as Post\nID: {}\nTitle: {}\nURL: {}\nDescription: {}", post.id, post.title, post.link, post.description.as_ref().unwrap_or(&"".to_string()))
                    } else {
                        String::from("Fail to add")
                    };

                    data.bot
                        .do_send(SendMessage::new(message.chat.id.to_string(), msg));
                }
            };

            if text.starts_with(r"/blog") {
                let re = Regex::new(r"/blog\s+([^\s]+)(\s*->\s*([^\s]+))?").unwrap();
                for cap in re.captures_iter(text.as_ref()) {
                    let url = &cap[1];
                    let rss = cap.get(3).map(|m| m.as_str().to_string());
                    let ret = Blog::new_blog(
                        NewBlogInput {
                            link: url.to_string(),
                        },
                        &data.postgres(),
                    );
                    info!("adding blog: {}", url);

                    let mut msg =
                        SendMessage::new(message.chat.id.to_string(), "Successfully add blog");
                    msg.reply_to_message_id = Some(message.message_id);
                    data.bot.do_send(msg);
                }
            };
        } else {
            let mut msg =
                SendMessage::new(message.chat.id.to_string(), "you are not in the white list");
            msg.reply_to_message_id = Some(message.message_id);
            data.bot.do_send(msg);
        };
    }
    "True"
}
