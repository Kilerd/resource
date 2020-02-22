
use serde::{Deserialize,Serialize};
use diesel::{Insertable, Queryable, query_builder::AsChangeset};
use crate::schema::reddits;


#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
pub struct Reddit {
    pub id: String,
    pub score: i32,
    pub title: String,
    pub selftext: Option<String>,
    pub author :String,
    pub permalink: String,
    pub url: String,
}