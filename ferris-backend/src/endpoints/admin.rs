use actix_web::{delete, error, get, post, put, web, HttpMessage, HttpRequest, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use crate::transfer::admin::{AdminLoginRequest, AdminLoginResponse, AdminLogoutRequest, AdminRemovePostRequest};

#[put("/admin")]
async fn login_admin(req: web::Json<AdminLoginRequest>) -> std::io::Result<HttpResponse> {
    println!("{:?}", req);
    // TODO: access database and create authtoken

    let authtoken = String::from("XXXXX");


    Ok(HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .json(AdminLoginResponse::new(authtoken))
    )
}

#[delete("/admin")]
async fn logout_admin(_: web::Json<AdminLogoutRequest>) -> std::io::Result<HttpResponse> {

    // TODO: remove authtoken from database

    Ok(HttpResponse::build(StatusCode::OK)
        .finish()
    )
}

#[delete("/admin/post")]
async fn admin_remove_post(_: web::Json<AdminRemovePostRequest>) -> std::io::Result<HttpResponse> {

    // TODO: verify authtoken, then remove post from specified board

    Ok(HttpResponse::build(StatusCode::OK)
        .finish()
    )
}