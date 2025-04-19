use serde::Deserialize;
use sqlx::{Database, Executor, Pool};
use crate::transfer::BoardInfo;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub boards: Vec<BoardInfo>,
    pub workers: usize,
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
}

#[derive(Copy, Clone)]
pub struct RuntimeConfig {
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
}

impl From<ServerConfig> for RuntimeConfig {
    fn from(config: ServerConfig) -> Self {
        Self {
            prevent_anonymous_posts: config.prevent_anonymous_posts,
            block_registrations: config.block_registrations,
        }
    }
}
