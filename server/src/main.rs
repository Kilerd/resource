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
use actix::SyncArbiter;
use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{
    middleware::{Logger, NormalizePath},
    web::{FormConfig, JsonConfig},
    App, HttpServer,
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::sync::Arc;
use telegram_typing_bot::bot::Bot;
use tera::compile_templates;
use time::Duration;

mod data;
mod models;
mod pg_pool;
mod routers;
mod schema;

embed_migrations!();

lazy_static! {
    static ref RANDOM_TOKEN_KEY: Vec<u8> = (0..32).map(|_| rand::random::<u8>()).collect();
    static ref TELEGRAM_WHITE_LSIT: Vec<i32> = std::env::var("TELEGRAM_WHITE_LIST")
        .unwrap_or_else(|_| String::from(""))
        .split(',')
        .map(|s| s.parse::<i32>().expect("cannot format as i32"))
        .collect();
    static ref TELEGRAM_RELEASE_CHANNEL: String =
        std::env::var("TELEGRAM_RELEASE_CHANNEL").expect("TELEGRAM_RELEASE_CHANNEL must be set");
}
fn main() {
    dotenv().ok();
    let sys = actix::System::new("resource");
    pretty_env_logger::init();
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL: database url must be set");
    info!("starting resource app on binding on 8000 port");
    let bot_addr = SyncArbiter::start(1, Bot::new);
    let data = AppData {
        pool: database_pool_establish(&database_url),
        tera: Arc::new(compile_templates!("templates/**/*.html")),
        bot: bot_addr,
    };

    embedded_migrations::run(&data.pool.get().expect("cannot get connection"))
        .expect("panic on embedded database migration");

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .data(JsonConfig::default().limit(256_000))
            .data(FormConfig::default().limit(256_000))
            .wrap(Logger::default())
            .wrap(Cors::default())
            .wrap(NormalizePath)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&RANDOM_TOKEN_KEY)
                    .name("auth-cookie")
                    .secure(false)
                    .max_age_time(Duration::days(3)),
            ))
            .service(routers::routes())
    })
    .bind(("0.0.0.0", 8000))
    .unwrap()
    .system_exit()
    .start();

    sys.run().expect("wrong on actix system run")
}
