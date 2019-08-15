use crate::model::Post;
use crate::session::Session;
use crate::GMsg;
use chrono::NaiveDate;
use futures::Future;
use seed::fetch::FetchObject;
use seed::prelude::*;
use seed::{Method, Request};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Model {
    session: Session,
    posts: Vec<Post>,
}

#[derive(Clone)]
pub enum Msg {
    FetchedPostsData(FetchObject<Vec<Post>>),
}

impl Model {
    pub fn view(&self) -> Node<Msg> {
        div!["posts"]
    }
}

fn request_posts() -> impl Future<Item = Msg, Error = Msg> {
    Request::new("http://localhost:8000/api/posts")
        .method(Method::Get)
        .fetch_json(Msg::FetchedPostsData)
}

pub fn init(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders.perform_cmd(request_posts());
    Model {
        session,
        ..Model::default()
    }
}
pub fn posts_render(posts: Vec<Post>) -> Vec<Node<Msg>> {
    let mut posts_group_by_day: BTreeMap<NaiveDate, Vec<Post>> = BTreeMap::new();
    for x in posts {
        let date = x.create_at.date().naive_utc();
        posts_group_by_day.entry(date).or_insert(vec![]).push(x);
    }
    let mut post_iter: Vec<(&NaiveDate, &Vec<Post>)> = posts_group_by_day.iter().collect();
    post_iter.reverse();

    post_iter
        .iter()
        .map(|(&daily, posts)| {
            section![
                class!["post-daily"],
                h3![daily.format("%B %d, %Y").to_string()],
                posts
                    .iter()
                    .map(|post| {
                        section![
                            class!["post"],
                            div![
                                class!["title"],
                                a![attrs! {At::Href => post.link}, post.title]
                            ],
                            if post.description.is_some() {
                                p![class!["description"], post.description.clone().unwrap()]
                            } else {
                                empty![]
                            }
                        ]
                    })
                    .collect::<Vec<Node<Msg>>>()
            ]
        })
        .collect()
}
pub fn view(model: &Model) -> Node<Msg> {
    web_sys::window()
        .expect("cannot find the window")
        .document()
        .expect("Can't find the window's document")
        .set_title("最新文章 - Resource.rs - 旨在提供一站式的 Rust 学习资源");

    //    let posts_elm : Vec<Node<Msg>> = model.posts.clone().iter().map(|post| div![ post.title]).collect();

    //    div![posts_render(model.posts.clone())]
    div![
        img![
            class!["bg"],
            attrs! {At::Src => "/public/rust.png", At::Alt=> "rust logo background"}
        ],
        section![
            class!["content"],
            header![
                img![
                    class!["logo"],
                    attrs! {At::Src => "/public/logo.png", At::Alt=> "resource.rs logo"}
                ],
                section![
                    class!["nav"],
                    h1!["最新文章", sup!["rs"]],
                    p![
                        class!["description"],
                        "尽可能持续为您提供最新最靠谱的 Rust 相关文章"
                    ],
                    nav![
                        a![attrs! {At::Href => "/"}, "首页"],
                        a![attrs! {At::Href => "/posts"}, "最新文章"],
                        // a![ attrs!{At::Href => "https://intellij-rust.github.io/"}, "相关博客"],
                        a![attrs! {At::Href => "/about"}, "关于我们"],
                    ]
                ],
            ],
            section![
                class!["center"],
                section![class!["part"], posts_render(model.posts.clone())]
            ]
        ]
    ]
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FetchedPostsData(data) => match data.response() {
            Ok(response) => model.posts = response.data,
            _ => {}
        },
    }
}
