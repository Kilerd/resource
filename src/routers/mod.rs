use crate::routers::{
    blog::{add_a_new_blogs, show_blogs},
    index::index_page,
    post::show_posts,
};
use actix_files::Files;
use actix_web::{web, HttpResponse, Scope};
use serde::{Deserialize, Serialize};

mod blog;
mod index;
mod post;

#[derive(Deserialize, Serialize)]
pub struct JsonResponse<T> {
    data: T,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    message: T,
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

    pub fn unauthorized(reason: impl Serialize) -> HttpResponse {
        HttpResponse::Unauthorized().json(&ErrorResponse { message: reason })
    }

    pub fn bad_gateway(reason: impl Serialize) -> HttpResponse {
        HttpResponse::BadGateway().json(&ErrorResponse { message: reason })
    }

    pub fn bad_request(reason: impl Serialize) -> HttpResponse {
        HttpResponse::BadRequest().json(&ErrorResponse { message: reason })
    }
}

pub fn routes() -> Scope {
    web::scope("/")
        .service(index_page)
        .service(
            web::scope("/blogs")
                .service(show_blogs)
                .service(add_a_new_blogs),
        )
        .service(web::scope("/posts").service(show_posts))
        .service(Files::new("/statics", "./templates/resources/"))
}
