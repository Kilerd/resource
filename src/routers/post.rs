use crate::{
    data::AppData,
    models::blog::{self, Blog},
    routers::AppResponder,
};
use actix_web::{
    get, post,
    web::{Data, Form, Json},
    Responder,
};
use tera::Context;

#[get("")]
pub fn show_posts(data: Data<AppData>) -> impl Responder {
    //    let blogs = Blog::get_all_confirmed_blogs(&data.postgres());
    let mut context = Context::new();
    //    context.insert("post", &blogs);
    AppResponder::html(data.render("posts.html", &context))
}
