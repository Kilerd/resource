use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use itertools::Itertools;
use pulldown_cmark::Options;
use r2d2::PooledConnection;
use serde::{Deserialize, Serialize};
use telegram_typing_bot::bot::Bot;
use tera::{Context, Tera};

use crate::pg_pool::{ManagedPgConn, Pool};

#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub tera: Arc<Tera>,
    pub bot: Arc<Bot>,
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
