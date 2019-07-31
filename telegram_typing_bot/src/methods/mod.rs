use crate::{bot::Bot, error::ApiResult};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;

//macro_rules! impl_method {
//    ($name:ident -> $return_type:ty) => {
//        impl $name {
//            fn request(
//                self,
//                bot: &Bot,
//            ) -> impl Future3<Output = Result<ApiResult<$return_type>, ()>> {
//                bot.request(stringify!($name), self)
//            }
//        }
//    };
//}

pub mod basic;
pub mod update;
//
//pub trait Method<'de>: Serialize + Deserialize<'de> + Debug + Sized {
//    type Type;
//
//    fn send(self, bot: &Bot) -> Box<Future3<Output = Result<ApiResult<Self::Type>, ()>>>
//    where
//        Self: 'static,
//        Self::Type: 'static + DeserializeOwned,
//    {
//        Box::new(bot.request("getMe", self))
//    }
//}
