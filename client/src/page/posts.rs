use seed::prelude::*;
use crate::{GMsg};
use crate::session::Session;
use futures::Future;
use crate::model::Post;
use seed::{Request, Method};
use seed::fetch::FetchObject;
use std::collections::BTreeMap;
use chrono::NaiveDate;

#[derive(Default)]
pub struct Model {
    session: Session,
    posts: Vec<Post>,
}

#[derive(Clone)]
pub enum  Msg {
    FetchedPostsData(FetchObject<Vec<Post>>)
}

impl Model {
    pub fn view(&self) -> Node<Msg> {
        div![
            "posts"
        ]
    }
}


fn request_posts() -> impl Future<Item=Msg, Error=Msg> {
    Request::new("http://localhost:8000/posts").method(Method::Get).fetch_json(Msg::FetchedPostsData)
}


pub fn init(session: Session, orders: &mut impl Orders<Msg, GMsg>) -> Model {
    orders.perform_cmd(request_posts());
//    orders
//        .perform_cmd(loading::notify_on_slow_load(
//            Msg::SlowLoadThresholdPassed,
//            Msg::Unreachable,
//        ))
//        .perform_cmd(request::article::load(
//            session.viewer().cloned(),
//            slug,
//            Msg::LoadArticleCompleted,
//        ))
//        .perform_cmd(request::comment::load_list(
//            session.viewer().cloned(),
//            slug,
//            Msg::LoadCommentsCompleted,
//        ));
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
        let mut post_iter: Vec<(&NaiveDate, &Vec<Post>)> =
            posts_group_by_day.iter().collect();
        post_iter.reverse();

    post_iter.iter().map(|(&daily, posts)| {
        section![ class!["post-daily"],
            h3![daily.format("%Y-%m-%d").to_string()],
            posts.iter().map(|post|{
                section![ class!["post"],
                    div![ class!["title"],
                        a![ post.title]
                    ]
                ]


            }).collect::<Vec<Node<Msg>>>()
        ]

    }).collect()
}
pub fn view(model: &Model) -> Node<Msg> {
    web_sys::window()
        .expect("cannot find the window")
        .document()
        .expect("Can't find the window's document")
        .set_title("Posts");

//    let posts_elm : Vec<Node<Msg>> = model.posts.clone().iter().map(|post| div![ post.title]).collect();
    div![
        posts_render(model.posts.clone())
    ]
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg, GMsg>) {
    match msg {
        Msg::FetchedPostsData(data) => {
            match data.response() {
                Ok(response) => {
                    model.posts = response.data
                },
                Err(err) => {seed::log(err)}
            }
        }
    }
}