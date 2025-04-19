use actix_web::{get, post, put, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use web::Json;
use crate::AppState;
use crate::transfer::post::{CreatePostReplyRequest, CreatePostReplyResponse, CreatePostRequest, CreatePostResponse, GetPostReplyRequest, GetPostReplyResponse, GetPostsRequest, GetPostsResponse};

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
async fn get_post_replies(req: Json<GetPostReplyRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let GetPostReplyRequest { parent, count, offset } = req.into_inner();

    // Eventually make it so that it only pulls recent active posts replies
    let result = crate::database::get_post_replies(&data.get_ref().db, parent as i64, count as i64, offset as i64).await
        .expect("unable to get posts");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(GetPostReplyResponse::new(result))
    )
}

#[post("/post")]
async fn create_post(req: Json<CreatePostRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let CreatePostRequest { board, category, image, text, auth_token} = req.into_inner();

    if auth_token.is_none() && data.get_ref().config.prevent_anonymous_posts {
        return Ok(HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE))
    }

    let result = crate::database::create_post(&data.get_ref().db, &board, &category, &image, &text, auth_token)
        .await
    .expect("unable to create post");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(CreatePostResponse::new(result)))
}

#[post("/post/reply")]
async fn create_post_reply(req: Json<CreatePostReplyRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let CreatePostReplyRequest { board, category, image, text, auth_token, parent } = req.into_inner();

    if auth_token.is_none() && data.get_ref().config.prevent_anonymous_posts {
        return Ok(HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE))
    }

    let result = crate::database::create_post_reply(&data.get_ref().db, &board, &category, &image, &text, parent, auth_token)
        .await
        .expect("unable to create post");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(CreatePostResponse::new(result)))
}