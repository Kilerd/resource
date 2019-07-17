use crate::{data::AppData, models::blog::Blog};
use actix_web::{
    get,
    web::{Data, Json},
    Responder,
};
use tera::Context;
use crate::routers::AppResponder;

#[get("/blogs")]
pub fn show_blogs(data: Data<AppData>) -> impl Responder {
    let blogs = Blog::read(&data.postgres());
    let mut context = Context::new();
    context.insert("blogs", &blogs);
    AppResponder::html(
        data.render("blogs.html", &context)
    )
}
