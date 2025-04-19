pub mod admin;
pub(crate) mod post;
pub(crate) mod user;

use actix_web::{get, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use ferris_shared::transfer::RootGetResponse;

#[get("/")]
async fn get_home(request: HttpRequest) -> std::io::Result<HttpResponse> {
    println!("{:?}", request);

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(RootGetResponse::default())
    )
}