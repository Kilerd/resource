use crate::models::blog::sql::NewBlog;
use chrono::{DateTime, NaiveDateTime};
use crypto::util::fixed_time_eq;
use diesel::{prelude::*, PgConnection};
use serde::Serialize;

#[derive(Queryable, Debug, Serialize)]
pub struct Blog {
    pub id: i32,
    pub title: Option<String>,
    pub link: String,
    pub description: Option<String>,
    pub rss: Option<String>,
    pub confirmed: bool,
    pub last_update_at: NaiveDateTime,
    pub last_rust_post_update_at: NaiveDateTime,
    pub create_at: NaiveDateTime,
}

impl Blog {
    pub fn get_all_confirmed_blogs(pg: &PgConnection) -> Vec<Blog> {
        use crate::schema::blog;
        blog::table
            .filter(blog::confirmed.eq(true))
            .order(blog::create_at.desc())
            .load::<Blog>(pg)
            .expect("cannot read blog list")
    }

    pub fn new_blog(new_blog: form::NewBlogInput, pg: &PgConnection) -> usize {
        use crate::schema::blog;
        let fetched_blogs: Vec<Blog> = blog::table
            .filter(blog::link.eq(&new_blog.link))
            .load::<Blog>(pg)
            .expect("cannot load blogs list");
        if fetched_blogs.len() == 0 {
            diesel::insert_into(blog::table)
                .values(NewBlog::new(new_blog))
                .execute(pg)
                .expect("cannot insert")
        } else {
            0
        }
    }
}

pub mod form {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct NewBlogInput {
        pub link: String,
    }
}

pub mod sql {
    use crate::{models::blog::form::NewBlogInput, schema::blog};
    use diesel::Insertable;

    #[derive(Insertable)]
    #[table_name = "blog"]
    pub struct NewBlog {
        pub link: String,
        pub confirmed: bool,
    }

    impl NewBlog {
        pub fn new(new_blog_input: NewBlogInput) -> NewBlog {
            NewBlog {
                link: new_blog_input.link,
                confirmed: false,
            }
        }
    }
}
