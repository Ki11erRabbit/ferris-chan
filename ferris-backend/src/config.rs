use serde::Deserialize;
use sqlx::{Database, Executor, Pool};
use crate::transfer::BoardInfo;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub boards: Vec<BoardInfo>,
}
