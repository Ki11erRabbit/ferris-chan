use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

impl LoginResponse {
    pub fn new(token: String) -> LoginResponse {
        LoginResponse { token }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LogoutRequest {
    pub token: String,
}