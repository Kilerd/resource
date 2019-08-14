use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub create_at: DateTime<Utc>,
    pub from_blog: Option<i32>,
    pub creator: String,
}