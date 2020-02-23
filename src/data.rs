use crate::pg_pool::{ManagedPgConn, Pool};
use r2d2::PooledConnection;
use std::sync::{Arc, Mutex};
use tera::{Context, Tera};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use pulldown_cmark::Options;
use std::ops::Deref;

#[derive(Serialize, Deserialize)]
pub struct MarkdownContent(String);

impl Deref for MarkdownContent {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl MarkdownContent {
    pub fn new(content: impl Into<String>) -> Self {
        let content = content.into();

        let mut html_output = String::new();
        let options = Options::all();
        let parser = pulldown_cmark::Parser::new_ext(&content, options);
        pulldown_cmark::html::push_html(&mut html_output, parser);

        MarkdownContent(html_output)

    }
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub index: toml::Value,
}

#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub tera: Arc<Tera>,
    pub data: Arc<Data>,
    pub pages: Arc<Mutex<HashMap<String,MarkdownContent>>>
}

impl AppData {
    pub fn postgres(&self) -> PooledConnection<ManagedPgConn> {
        let pool = self.pool.clone();
        pool.get().unwrap()
    }

    pub fn render(&self, template_name: &str, data: &Context) -> String {
        self.tera
            .render(template_name, data)
            .unwrap_or_else(|e| panic!("cannot render template {}, error: {}", template_name, e))
    }
}
