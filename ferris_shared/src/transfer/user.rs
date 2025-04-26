use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn new(email: String, password: String) -> Self {
        Self {
            email,
            password,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub is_admin: bool,
}

impl LoginResponse {
    pub fn new(token: String, is_admin: bool) -> LoginResponse {
        LoginResponse { token, is_admin }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LogoutRequest {
    pub token: String,
}