use serde::{Deserialize, Serialize};
use crate::transfer::BoardInfo;

#[derive(Deserialize, Serialize)]
pub struct AdminRemovePostRequest {
    pub post_id: usize,
    pub token: String,
    pub board_info: BoardInfo
}