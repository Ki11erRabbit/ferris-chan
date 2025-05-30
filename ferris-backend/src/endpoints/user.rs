use actix_web::{delete, post, web, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use crate::AppState;
use ferris_shared::transfer::user::{LoginRequest, LoginResponse, LogoutRequest, RegisterRequest};
use crate::database::DatabaseError;

#[post("/auth")]
async fn login_user(req: web::Json<LoginRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let LoginRequest { email, password } = req.into_inner();
    if email.is_empty() || password.is_empty() {
        return Ok(HttpResponse::new(StatusCode::UNAUTHORIZED));
    }

    let (auth_token, is_admin) = match data.get_ref().db.login_user(&email, &password).await {
        Ok(auth_token) => auth_token,
        Err(e) => {

            if let Ok(DatabaseError::UserOrPasswordDoesNotMatch) = e.downcast() {
                return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED)
                    .json(LoginResponse::error("Email or password does not match")));
            }

            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).finish())
        }
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(LoginResponse::new(auth_token, is_admin))
    )
}

#[delete("/auth/{token}")]
async fn logout_user(path: web::Path<(String)>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    let (token) = path.into_inner();

    match data.get_ref().db.logout_user(&token).await {
        Ok(_) => Ok(HttpResponse::new(StatusCode::NO_CONTENT)),
        Err(e) => {
            eprintln!("logout error: {}", e);
            Ok(HttpResponse::build(StatusCode::BAD_REQUEST).finish())
        }
    }
}

#[post("/auth/register")]
async fn register_user(req: web::Json<RegisterRequest>, data: web::Data<AppState>) -> std::io::Result<HttpResponse> {
    if data.get_ref().config.block_registrations {
        return Ok(HttpResponse::new(StatusCode::SERVICE_UNAVAILABLE))
    }

    let RegisterRequest { username, email, password } = req.into_inner();
    if email.is_empty() || password.is_empty() || username.is_empty() {
        return Ok(HttpResponse::new(StatusCode::UNAUTHORIZED));
    }

    let auth_token = match data.get_ref().db.register_user(&username, &email, &password).await {
        Ok(auth_token) => auth_token,
        Err(e) => {

            if let Ok(DatabaseError::UserAlreadyExists) = e.downcast::<DatabaseError>(){
                return Ok(HttpResponse::build(StatusCode::CONFLICT)
                    .json(LoginResponse::error("User already exists")));
            }


            return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).finish())
        }
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(LoginResponse::new(auth_token, false))
    )
}