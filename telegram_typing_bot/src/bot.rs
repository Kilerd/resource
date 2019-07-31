//use std::env;
//
//use futures::{
//    compat::Future01CompatExt,
//    future::{Future as Future3, FutureExt, TryFuture, TryFutureExt},
//    task::SpawnExt,
//};
//use reqwest::r#async::{Client, Response};
//use serde::{Deserialize, Serialize};
//
//use crate::error::{ApiResult, BotResult};
//
//use futures01::Future as Future1;
//use std::fmt::Debug;
//
use crate::methods::basic::SendMessage;
use actix::prelude::*;

pub struct Bot {
    pub secret_key: String,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            secret_key: std::env::var("TELEGRAM_BOT_SECRET_KEY")
                .expect("need to set TELEGRAM_BOT_SECRET_KEY as environment variable"),
        }
    }
}

impl Actor for Bot {
    type Context = SyncContext<Self>;
}

//
//impl Bot {
//    pub fn new(token: impl Into<String>) -> Self {
//        Self {
//            token: token.into(),
//        }
//    }
//
//    pub fn request<P, T>(
//        &self,
//        method: impl Into<String>,
//        payload: P,
//    ) -> impl Future3<Output = Result<ApiResult<T>, ()>>
//    where
//        P: Serialize + Debug,
//        T: serde::de::DeserializeOwned,
//    {
//        Client::new()
//            .post(dbg!(format!(
//                "https://api.telegram.org/bot{}/{}",
//                env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"),
//                method.into()
//            )
//            .as_str()))
//            .json(dbg!(&payload))
//            .send()
//            .and_then(|mut response: Response| {
//                dbg!(response.status());
//                response.json::<ApiResult<T>>()
//            })
//            .map_err(|e| {
//                dbg!(e);
//                ()
//            })
//            .compat()
//    }
//}
//
//#[cfg(test)]
//mod test {
//    use futures01::Future as Future1;
//    use hyper::{service::service_fn_ok, Body, Request, Response, Server};
//    #[test]
//    fn test_binding_server() {
//        let addr = ([127, 0, 0, 1], 8000).into();
//
//        let server = Server::bind(&addr)
//            .serve(|| {
//                // This is the `Service` that will handle the connection.
//                // `service_fn_ok` is a helper to convert a function that
//                // returns a Response into a `Service`.
//                service_fn_ok(move |_: Request<Body>| Response::new(Body::from("Hello World!")))
//            })
//            .map_err(|e| eprintln!("server error: {}", e));
//
//        println!("Listening on http://{}", addr);
//
//        hyper::rt::run(server);
//    }
//}
