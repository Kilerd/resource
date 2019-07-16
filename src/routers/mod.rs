use actix_web::{web, Scope};
use crate::routers::index::index_page;

mod index;

pub fn routes() -> Scope {
    web::scope("/").service(index_page)
}
