use actix_web::{Scope, web};

pub fn routes() -> Scope {
    web::scope("/")
}