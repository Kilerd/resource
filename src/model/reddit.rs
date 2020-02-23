
use serde::{Deserialize,Serialize};
use diesel::{Insertable, Queryable, query_builder::AsChangeset};
use crate::schema::reddits;
use chrono::{DateTime, Utc};


#[derive(Queryable, Debug, Serialize, Insertable, AsChangeset)]
pub struct Reddit {
    pub id: String,
    pub score: i32,
    pub title: String,
    pub selftext: Option<String>,
    pub author :String,
    pub permalink: String,
    pub url: String,
    pub create_time: DateTime<Utc>,
    pub telegram_message_id: Option<String>,
}