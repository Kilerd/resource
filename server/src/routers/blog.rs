use crate::{
    data::AppData,
    models::blog::{self, Blog},
    routers::AppResponder,
};
use actix_web::{
    get, post,
    web::{Data, Json},
    Responder,
};
use tera::Context;

#[get("")]
pub fn show_blogs(data: Data<AppData>) -> impl Responder {
    let blogs = Blog::get_all_confirmed_blogs(&data.postgres());
    let mut context = Context::new();
    context.insert("blogs", &blogs);
    AppResponder::html(data.render("blogs.html", &context))
}

#[post("")]
pub fn add_a_new_blogs(
    data: Data<AppData>,
    input: Json<blog::form::NewBlogInput>,
) -> impl Responder {
    let insert_number = Blog::new_blog(input.0, &data.postgres());
    let msg = if insert_number == 0 {
        "the blog is already in the list"
    } else {
        "successfully add into the list"
    };
    AppResponder::json(msg)
}
