use actix_web::{delete, put, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use sqlx::{SqlitePool};
use crate::transfer::user::{LoginRequest, LoginResponse, LogoutRequest};

#[put("/auth")]
async fn login_user(req: web::Json<LoginRequest>, data: web::Data<SqlitePool>) -> std::io::Result<HttpResponse> {
    let LoginRequest { email, password } = req.into_inner();
    if email.is_empty() || password.is_empty() {
        return Ok(HttpResponse::new(StatusCode::UNAUTHORIZED));
    }

    let auth_token = match crate::database::login_user(data.get_ref(), &email, &password).await {
        Ok(auth_token) => auth_token,
        Err(e) => {
            eprintln!("login_user error: {}", e);
            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).finish())
        }
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(LoginResponse::new(auth_token))
    )
}

#[delete("/auth")]
async fn logout_user(req: web::Json<LogoutRequest>, data: web::Data<SqlitePool>) -> std::io::Result<HttpResponse> {
    let LogoutRequest{ token } = req.into_inner();

    match crate::database::logout_user(data.get_ref(), &token).await {
        Ok(_) => Ok(HttpResponse::new(StatusCode::NO_CONTENT)),
        Err(e) => {
            eprintln!("logout error: {}", e);
            Ok(HttpResponse::build(StatusCode::BAD_REQUEST).finish())
        }
    }
}
