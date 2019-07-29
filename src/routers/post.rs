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
use chrono::{NaiveDate, NaiveDateTime};
use std::collections::{BTreeMap, HashMap};
use tera::Context;

#[get("")]
pub fn show_posts(data: Data<AppData>) -> impl Responder {
    let posts = Post::get_all_posts(&data.postgres());
    let mut posts_group_by_day: BTreeMap<NaiveDate, Vec<(Post, Option<Blog>)>> = BTreeMap::new();
    for x in posts {
        let date = x.0.create_at.date();
        posts_group_by_day.entry(date).or_insert(vec![]).push(x);
    }
    let mut post_iter: Vec<(&NaiveDate, &Vec<(Post, Option<Blog>)>)> =
        posts_group_by_day.iter().collect();
    let mut context = Context::new();
    context.insert("posts", &post_iter);
    AppResponder::html(data.render("posts.html", &context))
}
