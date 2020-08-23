use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

pub mod api;
pub mod reddit;

#[derive(Deserialize, Serialize)]
pub struct JsonResponse<T> {
    data: T,
}

pub struct AppResponder;

impl AppResponder {
    pub fn json(data: impl Serialize) -> HttpResponse {
        HttpResponse::Ok()
            .header(
                http::header::CONTENT_TYPE,
                "application/json; charset=utf-8",
            )
            .json(JsonResponse { data })
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(reddit::reddit_rending_api);
}
