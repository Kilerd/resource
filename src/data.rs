use crate::pg_pool::{ManagedPgConn, Pool};
use actix::Addr;
use r2d2::PooledConnection;
use std::sync::Arc;
use telegram_typing_bot::bot::Bot;
use tera::{Context, Tera};

#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub tera: Arc<Tera>,
    pub bot: Addr<Bot>,
}

impl AppData {
    pub fn postgres(&self) -> PooledConnection<ManagedPgConn> {
        let pool = self.pool.clone();
        pool.get().unwrap()
    }

    pub fn render(&self, template_name: &str, data: &Context) -> String {
        self.tera
            .render(template_name, data)
            .unwrap_or_else(|_| panic!("cannot render template {}", template_name))
    }
}
