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
}

#[derive(Clone)]
pub enum Msg {}

impl Model {
    pub fn view(&self) -> Node<Msg> {
        div!["posts"]
    }
}

pub fn init(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    Model {
        session,
        ..Model::default()
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    web_sys::window()
        .expect("cannot find the window")
        .document()
        .expect("Can't find the window's document")
        .set_title("关于我们 - Resource.rs - 旨在提供一站式的 Rust 学习资源");
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
                    h1!["关于我们", sup!["rs"]],
                    p![class!["description"], "我们的那些事"],
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
                section![class!["part"], md![include_str!("about.md")]]
            ]
        ]
    ]
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {}
}
