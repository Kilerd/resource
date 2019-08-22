use crate::data::AppData;
use crate::models::post::Post;
use crate::schema::post;
use actix_web::{get, web, HttpResponse, Responder};
use diesel::prelude::*;
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use std::collections::HashMap;

#[get("/rss")]
pub fn rss_for_latest_posts(data: web::Data<AppData>) -> impl Responder {
    let mut namespaces: HashMap<String, String> = HashMap::new();
    namespaces.insert(
        "dc".to_string(),
        "http://purl.org/dc/elements/1.1/".to_string(),
    );
    namespaces.insert(
        "content".to_string(),
        "http://purl.org/rss/1.0/modules/content/".to_string(),
    );
    namespaces.insert(
        "atom".to_string(),
        "http://www.w3.org/2005/Atom".to_string(),
    );
    namespaces.insert(
        "media".to_string(),
        "http://search.yahoo.com/mrss/".to_string(),
    );

    let posts: Vec<Post> = post::table
        .order(post::create_at.desc())
        .limit(20)
        .load::<Post>(&data.postgres())
        .expect("cannot load posts");

    dbg!(&posts);
    let items: Vec<Item> = posts
        .iter()
        .map(|post| {
            ItemBuilder::default()
                .title(post.title.clone())
                .link(post.link.clone())
                .description(post.description.clone())
                .content(post.description.clone())
                .pub_date(post.create_at.to_string())
                .build()
                .unwrap()
        })
        .collect();

    let channel: Channel = ChannelBuilder::default()
        .title("Resource.rs")
        .description("Rust 相关的最新资源")
        .generator("Resource.rs".to_string())
        .link("https://resource.rs")
        .items(items)
        .namespaces(namespaces)
        .build()
        .unwrap();
    HttpResponse::Ok()
        .content_type("text/xml; charset=utf-8")
        .body(channel.to_string())
}
