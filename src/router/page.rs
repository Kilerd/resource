use actix_web::Responder;
use actix_web::{get};
use crate::router::AppResponder;
use crate::data::{AppData, MarkdownContent};
use actix_web::web::{Data, Path};
use tera::Context;

#[get("/")]
pub async fn index(data: Data<AppData>) -> impl Responder {
    let mut context = Context::new();
    context.insert("data", &data.data.index);
    AppResponder::html(data.render("index.html", &context))
}

#[get("{url}")]
pub async fn get_article_by_url(
    url: Path<String>,
    data: Data<AppData>,
) -> impl Responder {
    let url = url.into_inner();
    let mut pages = data.pages.lock().unwrap();

    let option = pages.get(&url);
    if option.is_none() {
        let path1 = std::path::Path::new("./page");
        let buf = path1.join(format!("{}.md", &url));
        let result = std::fs::read_to_string(&buf);
        match result {
            Ok(content) => {
                info!("load page data: {}", &buf.to_str().unwrap());
                let content = MarkdownContent::new(content);
                pages.insert(url.clone(), content);
            }
            Err(_) => {
                error!("cannot find page: {}", &buf.to_str().unwrap());
                return AppResponder::not_found()
            }
        }
    }
    let option = pages.get(&url);


    if let Some(page) = option {
        let mut context = Context::new();
        context.insert("page", page);
        AppResponder::html(data.render("page.html", &context))
    }else {
        AppResponder::not_found()
    }
}