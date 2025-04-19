use actix_web::{get, put, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use web::Json;
use crate::AppState;
use crate::transfer::post::{GetPostReplyRequest, GetPostReplyResponse, GetPostsRequest, GetPostsResponse};

#[get("/post")]
async fn get_posts(req: Json<GetPostsRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let GetPostsRequest { board, category, offset, count } = req.into_inner();

    // Eventually make it so that it only pulls recent active posts
    let result = crate::database::get_posts(&data.get_ref().db, &board, &category, count as i64, offset as i64).await
        .expect("unable to get posts");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(GetPostsResponse::new(result))
    )
}

#[get("/post/reply")]
async fn get_post_replies(_: Json<GetPostReplyRequest>) -> std::io::Result<HttpResponse> {

    // TODO! look into database and pull n replies from the database starting offset from the start
    // Eventually make it so that it only pulls recent active posts replies

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(GetPostReplyResponse::default())
    )
}