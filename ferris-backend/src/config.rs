use serde::Deserialize;
use sqlx::{Database, Executor, Pool};
use crate::transfer::BoardInfo;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub boards: Vec<BoardInfo>,
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
}
