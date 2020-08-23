extern crate openssl;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;

use crate::{data::AppData, pg_pool::database_pool_establish};
use actix_cors::Cors;
use actix_web::{
    middleware::{Logger, NormalizePath},
    web::{FormConfig, JsonConfig},
    App, HttpServer,
};
use dotenv::dotenv;
use futures::future::abortable;
use once_cell::sync::Lazy;
use std::sync::Arc;
use telegram_typing_bot::bot::Bot;

mod data;
mod model;
mod pg_pool;
mod router;
mod schema;

embed_migrations!();

static TOKEN_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var("TOKEN_KEY")
        .map(|token| Vec::from(token.as_bytes()))
        .unwrap_or_else(|_| (0..32).into_iter().map(|_| rand::random::<u8>()).collect())
});
static TELEGRAM_RESOURCE_CHANNEL: Lazy<String> = Lazy::new(|| {
    std::env::var("TELEGRAM_RESOURCE_CHANNEL").expect("TELEGRAM_RESOURCE_CHANNEL must be set")
});
static DATABASE_URL: Lazy<String> =
    Lazy::new(|| std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

#[actix_rt::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();

    info!("TOKEN_KEY is initialized as {:?}", *TOKEN_KEY);

    let bot = Bot::new();

    let data = AppData {
        pool: database_pool_establish(&DATABASE_URL),
        bot: Arc::new(bot),
    };

    embedded_migrations::run(&data.pool.get().expect("cannot get connection"))
        .expect("panic on embedded database migration");

    let (fut, handler) = abortable(router::reddit::looping_fetch(data.clone()));
    tokio::spawn(fut);

    info!("resource is listening on 0.0.0.0:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .data(data.clone())
            .data(JsonConfig::default().limit(256_000))
            .data(FormConfig::default().limit(256_000))
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(NormalizePath {})
            .configure(router::routes)
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .run()
    .await
    .unwrap();

    info!("application exit");
    handler.abort();
}
