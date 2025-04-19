use actix_web::{delete, error, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use crate::transfer::admin::AdminRemovePostRequest;

#[delete("/admin/post")]
async fn admin_remove_post(_: web::Json<AdminRemovePostRequest>) -> std::io::Result<HttpResponse> {

    // TODO: verify authtoken, then remove post from specified board

    Ok(HttpResponse::build(StatusCode::OK)
        .finish()
    )
}