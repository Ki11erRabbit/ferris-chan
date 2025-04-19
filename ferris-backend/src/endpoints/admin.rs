use actix_web::{delete, web, HttpResponse};
use actix_web::http::StatusCode;
use ferris_shared::transfer::admin::AdminRemovePostRequest;

#[delete("/admin/post")]
async fn admin_remove_post(_: web::Json<AdminRemovePostRequest>) -> std::io::Result<HttpResponse> {

    // TODO: verify authtoken, then remove post from specified board

    Ok(HttpResponse::build(StatusCode::OK)
        .finish()
    )
}