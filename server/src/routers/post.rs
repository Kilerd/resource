use crate::{
    data::AppData,
    models::{blog::Blog, post::Post},
    routers::AppResponder,
};
use actix_web::web::Json;
use actix_web::{get, web::Data, Responder};
use chrono::NaiveDate;
use std::collections::BTreeMap;
use tera::Context;

#[get("")]
pub fn show_posts(data: Data<AppData>) -> impl Responder {
    let mut ret = vec![];
    for item in Post::get_all_posts(&data.postgres()) {
        ret.push(item.0);
    }
    Json(ret)
}
