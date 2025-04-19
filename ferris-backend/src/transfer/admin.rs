use serde::{Deserialize, Serialize};
use crate::transfer::BoardInfo;

#[derive(Deserialize, Serialize, Debug)]
pub struct AdminLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct AdminLoginResponse {
    pub token: String,
}

impl AdminLoginResponse {
    pub fn new(token: String) -> AdminLoginResponse {
        AdminLoginResponse { token }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AdminLogoutRequest {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
pub struct AdminRemovePostRequest {
    pub post_id: u64,
    pub token: String,
    pub board_info: BoardInfo
}