use crate::{data::AppData, routers::AppResponder};
use actix_web::{get, web::Data, Responder};
use tera::Context;

#[get("")]
pub fn index_page(data: Data<AppData>) -> impl Responder {
    AppResponder::html(data.render("index.html", &Context::new()))
}
