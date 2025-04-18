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
async fn get_home(request: HttpRequest, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    println!("{:?}", request);
    let RuntimeConfig { name, logo, boards, .. } = &data.get_ref().config;

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(RootGetResponse::new(name.clone(), logo.clone(), boards.clone()))
    )
}