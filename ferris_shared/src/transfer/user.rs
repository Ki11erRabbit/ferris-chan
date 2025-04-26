use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl RegisterRequest {
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            username,
            email,
            password,
        }
    }
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
#[serde(untagged)]
pub enum LoginResponse {
    Success {
        token: String,
        is_admin: bool,
    },
    Error {
        message: String,
    }
}

impl LoginResponse {
    pub fn new(token: String, is_admin: bool) -> LoginResponse {
        LoginResponse::Success { token, is_admin }
    }

    pub fn error<M: AsRef<str>>(message: M) -> Self {
        LoginResponse::Error { message: message.as_ref().to_string() }
    }
}

#[derive(Deserialize, Serialize)]
pub struct LogoutRequest {
    pub token: String,
}