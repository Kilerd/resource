use chrono::{DateTime, NaiveDateTime};
use diesel::{prelude::*, PgConnection};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Blog {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub last_update_at: NaiveDateTime,
    pub last_rust_post_update_at: NaiveDateTime,
    pub create_at: NaiveDateTime,
    pub rss: Option<String>,
}

impl Blog {
    pub fn read(pg: &PgConnection) -> Vec<Blog> {
        use crate::schema::blog;
        blog::table
            .order(blog::create_at.desc())
            .load::<Blog>(pg)
            .expect("cannot read blog list")
    }
}
