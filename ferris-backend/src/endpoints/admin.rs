use actix_web::{delete, web, HttpRequest, HttpResponse};
use actix_web::http::StatusCode;
use ferris_shared::transfer::admin::AdminRemovePostResponse;
use crate::AppState;

#[delete("/admin/post/{post_id}")]
async fn admin_remove_post(req: HttpRequest, path: web::Path<(i64)>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {

    let token = req.headers().get("Authorization").unwrap();
    let mut token = token.to_str().unwrap().to_string();
    let token = token.drain(7..).collect::<String>();

    let post_id = path.into_inner();

    let Ok(_) = data.get_ref().db.delete_post(post_id, &token).await else {
        return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED)
            .json(AdminRemovePostResponse::error(String::from("This action requires an authorized auth token"))));
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .json(AdminRemovePostResponse::success(post_id as usize))
    )
}