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
    //    let posts = Post::get_all_posts(&data.postgres());
    //    let mut posts_group_by_day: BTreeMap<NaiveDate, Vec<(Post, Option<Blog>)>> = BTreeMap::new();
    //    for x in posts {
    //        let date = x.0.create_at.date().naive_utc();
    //        posts_group_by_day.entry(date).or_insert(vec![]).push(x);
    //    }
    //    let mut post_iter: Vec<(&NaiveDate, &Vec<(Post, Option<Blog>)>)> =
    //        posts_group_by_day.iter().collect();
    //    let mut context = Context::new();
    //    context.insert("posts", &post_iter);
    //    AppResponder::html(data.render("posts.html", &context))
    let mut ret = vec![];
    for item in Post::get_all_posts(&data.postgres()) {
        ret.push(item.0);
    }
    Json(ret)
}
