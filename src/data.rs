use crate::pg_pool::{ManagedPgConn, Pool};
use r2d2::PooledConnection;
use std::sync::Arc;
use tera::{Context, Tera};

#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub tera: Arc<Tera>,
}

impl AppData {
    pub fn postgres(&self) -> PooledConnection<ManagedPgConn> {
        let pool = self.pool.clone();
        pool.get().unwrap()
    }

    pub fn render(&self, template_name: &str, data: &Context) -> String {
        self.tera
            .render(template_name, data)
            .expect(&format!("cannot render template {}", template_name))
    }
}
