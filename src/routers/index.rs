use crate::data::AppData;
use actix_web::{get, web::Data, Responder};
use tera::Context;

#[get("")]
pub fn index_page(data: Data<AppData>) -> impl Responder {

    data.render("index.html", &Context::new())
}
