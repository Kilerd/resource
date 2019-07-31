use crate::{
    bot::Bot,
    error::ApiResult,
    typing::{
        ForceReply, InlineKeyboardButton, Message, ReplyKeyboardMarkup, ReplyKeyboardRemove, User,
    },
};
use actix::Handler;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{env, future::Future as Future3};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMe {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(Vec<Vec<InlineKeyboardButton>>),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessage {
    pub chat_id: String,
    pub text: String,
    pub parse_mod: Option<String>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendMessage {
    pub fn new(chat_id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mod: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardMessage {
    pub chat_id: String,
    pub from_chat_id: String,
    pub disable_notification: Option<bool>,
    pub message_id: i32,
}

impl actix::Message for SendMessage {
    type Result = ();
}

impl Handler<SendMessage> for Bot {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("doing sendmessage");
        reqwest::Client::new()
            .post(
                format!(
                    "https://api.telegram.org/bot{}/sendMessage",
                    self.secret_key
                )
                .as_str(),
            )
            .json(&msg)
            .send()
            .map_err(|e| {
                dbg!(&e);
                e
            });

        ()
    }
}

//impl GetMe {
//    fn request(self, bot: &Bot) -> impl Future3<Output = Result<ApiResult<User>, ()>> {
//        bot.request("getMe", self)
//    }

//    fn request_(self) -> impl Future1<Item=ApiResult<User>, Error=actix_web::client::SendRequestError> {
//        Client::new()
//            .post(format!(
//                "https://api.telegram.org/bot{}/{}",
//                env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"),
//                "getMe"))
//            .timeout(std::time::Duration::from_secs(10))
//            .send_json(&self)
//            .map(|mut response| {
//                response.json::<ApiResult<User>>()
//            }).wait()
//    }
//}
//
//impl_method!(SendMessage -> Message);
//impl_method!(ForwardMessage -> Message);
//
//#[cfg(test)]
//mod tests {
//    use crate::{
//        bot::Bot,
//        methods::{basic::GetMe, basic::SendMessage},
//    };
//    use crate::error::ApiResult;
//    use crate::typing::User;
//    use futures::future::{TryFutureExt, FutureExt};
//
//    #[test]
//    fn should_get_me_work() {
//        let x = async {
//            let bot = Bot::new(std::env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"));
//            let get_me = GetMe {};
//            let x = get_me.request(&bot);
//            let a = x.await;
//            println!("{:?}", a);
//            assert_eq!(true, a.unwrap().ok);
//        };
//        tokio::run(x.unit_error().boxed().compat());
//    }
//
//    #[test]
//    fn should_send_message_work() {
//        let x = async {
//            let bot = Bot::new(std::env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"));
//            let send_message = SendMessage::new(std::env::var("SEND_MESSAGE_CHAT_ID").expect("need SEND_MESSAGE_CHAT_ID"), "hello test");
//            let x = send_message.request(&bot);
//            let a = x.await;
//            dbg!(&a);
//            assert_eq!(true, a.unwrap().ok);
//            ()
//        };
//        tokio::run(x.unit_error().boxed().compat());
//    }
//}
