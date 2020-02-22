#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
extern crate openssl;

use std::sync::{Arc, Mutex};
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::{Logger, NormalizePath}, web::{FormConfig, JsonConfig}};
use tera::{Tera};
use once_cell::sync::Lazy;
use dotenv::dotenv;
use crate::{data::AppData, pg_pool::database_pool_establish};
use crate::data::Data;
use futures::future::abortable;

mod data;
mod pg_pool;
mod schema;
mod router;
mod model;

embed_migrations!();

const TOKEN_KEY: Lazy<Vec<u8>> = Lazy::new(|| {
    std::env::var("TOKEN_KEY")
        .map(|token| Vec::from(token.as_bytes()))
        .unwrap_or_else(|_| (0..32).into_iter().map(|_| rand::random::<u8>()).collect())
});


#[actix_rt::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();

    let database_url = std::env::var("DATABASE_URL").expect("database_url must be set");


    let data = AppData {
        pool: database_pool_establish(&database_url),
        tera: Arc::new(Tera::new("templates/**/*.html").unwrap()),
        data: Arc::new(Data {
            index: toml::from_str(include_str!("../data/index.toml")).unwrap()
        }),
        pages: Arc::new(Mutex::new(Default::default()))
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