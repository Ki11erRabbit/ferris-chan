use actix_web::{get, post, put, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use web::Json;
use crate::AppState;
use ferris_shared::transfer::post::{CreatePostReplyRequest, CreatePostReplyResponse, CreatePostRequest, CreatePostResponse, GetPostReplyRequest, GetPostReplyResponse, GetPostsRequest, GetPostsResponse};
use crate::database::DatabaseError;

#[get("/post/{category}/{board}/{count}/{offset}")]
async fn get_posts(path: web::Path<(String, String, i64, i64)>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let (category, board, count, offset) = path.into_inner();
    // Eventually make it so that it only pulls recent active posts

    let category = urlencoding::decode(category.as_str()).unwrap();
    let board = urlencoding::decode(board.as_str()).unwrap();

    let result = data.get_ref().db.get_posts(&board, &category, count, offset).await
        .expect("unable to get posts");

    Ok(HttpResponse::build(StatusCode::OK)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .content_type(ContentType::json())
        .json(GetPostsResponse::new(result))
    )
}

#[get("/post-reply/{parent}/{count}/{offset}")]
async fn get_post_replies(path: web::Path<(i64, i64, i64)>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let (parent, count, offset) = path.into_inner();

    // Eventually make it so that it only pulls recent active posts replies
    let result = data.get_ref().db.get_post_replies(parent, count, offset).await
        .expect("unable to get posts");

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(GetPostReplyResponse::new(result))
    )
}

#[get("/post/{post_id}")]
async fn get_post_image(path: web::Path<(i64)>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let (post_id) = path.into_inner();

    let result = data.get_ref().db.get_post_image(post_id).await
        .expect("unable to get posts");

    Ok(HttpResponse::build(StatusCode::OK)
        .append_header(("Access-Control-Allow-Origin", "*"))
        .content_type(ContentType::png())
        .body(result)
    )
}

#[post("/post")]
async fn create_post(req: Json<CreatePostRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    log::debug!("Create post request : {:?}", &req);
    let CreatePostRequest { board, category, image, text, auth_token, alt_text} = req.into_inner();


    if auth_token.is_none() && data.get_ref().config.prevent_anonymous_posts {
        return Ok(HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE))
    }

    let result = data.get_ref().db.create_post(&board, &category, &image, &alt_text, &text, auth_token)
        .await;

    match result {
        Ok(post) => {
            Ok(HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::json())
                .json(CreatePostResponse::new(post)))
        }
        Err(e) => {
            if let Ok(DatabaseError::AuthTokenExpired) = e.downcast() {
                Ok(HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .json(CreatePostResponse::error("Token expired")))
            } else {
                Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .json(CreatePostResponse::error("Internal server error")))
            }
        }
    }

}

#[post("/post/reply")]
async fn create_post_reply(req: Json<CreatePostReplyRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let CreatePostReplyRequest { board, category, image, text, auth_token, parent, alt_text } = req.into_inner();

    if auth_token.is_none() && data.get_ref().config.prevent_anonymous_posts {
        return Ok(HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE))
    }

    let result = data.get_ref().db.create_post_reply(&board, &category, &image, &alt_text, &text, parent, auth_token)
        .await;

    match result {
        Ok(post) => {
            Ok(HttpResponse::build(StatusCode::OK)
                .content_type(ContentType::json())
                .json(CreatePostResponse::new(post)))
        }
        Err(e) => {
            if let Ok(DatabaseError::AuthTokenExpired) = e.downcast() {
                Ok(HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .json(CreatePostResponse::error("Token is expired")))
            } else {
                Ok(HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .json(CreatePostResponse::error("Internal server error")))
            }
        }
    }


}