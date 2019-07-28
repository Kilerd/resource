use crate::{
    data::AppData,
    models::{
        blog::{self, Blog},
        post::Post,
    },
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
    let posts = Post::get_all_posts(&data.postgres());
    let mut context = Context::new();
    context.insert("posts", &posts);
    AppResponder::html(data.render("posts.html", &context))
}
