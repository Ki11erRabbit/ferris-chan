use serde::{Deserialize, Serialize};
use crate::transfer::BoardInfo;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum AdminRemovePostResponse {
    Success {
        post_id: usize
    },
    Error {
        message: String
    }
}

impl AdminRemovePostResponse {
    pub fn success(post_id: usize) -> AdminRemovePostResponse {
        AdminRemovePostResponse::Success { post_id }
    }

    pub fn error(message: String) -> AdminRemovePostResponse {
        AdminRemovePostResponse::Error { message }
    }
}