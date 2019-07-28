use crate::models::blog::{sql::NewBlog, Blog};
use chrono::{DateTime, NaiveDateTime};
use crypto::util::fixed_time_eq;
use diesel::{prelude::*, PgConnection};
use serde::Serialize;
use telegram_typing_bot::typing::User;

#[derive(Queryable, Debug, Serialize)]
//#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub create_at: NaiveDateTime,
    pub from_blog: Option<i32>,
    pub creator: String,
}

impl Post {
    pub fn get_all_posts(pg: &PgConnection) -> Vec<(Post, Option<Blog>)> {
        use crate::schema::*;
        post::table
            .left_outer_join(blog::table)
            .order(post::create_at.desc())
            .load::<(Post, Option<Blog>)>(pg)
            .expect("cannot load posts")
    }

    pub fn insert(new_post: sql::NewPost, pg: &PgConnection) -> Option<Post> {
        use crate::schema::post;
        diesel::insert_into(post::table)
            .values(new_post)
            .get_result(pg)
            .ok()
    }
}

pub mod sql {
    use crate::schema::post;
    use diesel::Insertable;

    #[derive(Insertable)]
    #[table_name = "post"]
    pub struct NewPost {
        pub title: String,
        pub link: String,
        pub description: Option<String>,
        pub from_blog: Option<i32>,
        pub creator: String,
    }
}
