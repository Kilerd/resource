use std::sync::Arc;

use r2d2::PooledConnection;
use telegram_typing_bot::bot::Bot;

use crate::pg_pool::{ManagedPgConn, Pool};

#[derive(Clone)]
pub struct AppData {
    pub pool: Pool,
    pub bot: Arc<Bot>,
}

impl AppData {
    pub fn postgres(&self) -> PooledConnection<ManagedPgConn> {
        let pool = self.pool.clone();
        pool.get().unwrap()
    }
}
