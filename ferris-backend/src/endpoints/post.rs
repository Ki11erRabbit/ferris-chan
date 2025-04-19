use actix_web::{get, put, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use web::Json;
use crate::transfer::post::{GetPostReplyRequest, GetPostReplyResponse, GetPostsRequest, GetPostsResponse};

#[get("/post")]
async fn get_posts(_: Json<GetPostsRequest>) -> std::io::Result<HttpResponse> {

    // TODO! look into database and pull n posts from the database starting offset from the end
    // Eventually make it so that it only pulls recent active posts

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(GetPostsResponse::default())
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