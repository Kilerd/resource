use actix_web::middleware::NormalizePath;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};

mod api;
mod page;

#[derive(Deserialize, Serialize)]
pub struct JsonResponse<T> {
    data: T,
}
pub struct AppResponder;

impl AppResponder {
    pub fn html(content: impl Into<String>) -> HttpResponse {
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(content.into())
    }

    pub fn json(data: impl Serialize) -> HttpResponse {
        HttpResponse::Ok()
            .header(
                http::header::CONTENT_TYPE,
                "application/json; charset=utf-8",
            )
            .json(JsonResponse { data })
    }

    pub fn text(content: impl Into<String>) -> HttpResponse {
        HttpResponse::Ok().body(content.into())
    }

    pub fn redirect(to: impl Into<String>) -> HttpResponse {
        HttpResponse::Found()
            .header(http::header::LOCATION, to.into())
            .finish()
    }

    pub fn redirect_permanently(to: impl Into<String>) -> HttpResponse {
        HttpResponse::MovedPermanently()
            .header(http::header::LOCATION, to.into())
            .finish()
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::NotFound().finish()
    }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(actix_files::Files::new(
            "/statics",
            "./templates/statics/",
        ))
        .service(page::index)
        .service(page::get_article_by_url)
    ;
}