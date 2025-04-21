pub mod admin;
pub(crate) mod post;
pub(crate) mod user;

use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use ferris_shared::transfer::RootGetResponse;
use crate::AppState;
use crate::config::RuntimeConfig;

#[get("/")]
async fn get_home(_: HttpRequest, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let RuntimeConfig { name, logo, boards, categories, .. } = &data.get_ref().config;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(RootGetResponse::new(name.clone(), logo.clone(), categories.clone(), boards.clone()))
    )
}