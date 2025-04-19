use serde::Deserialize;
use sqlx::{Database, Executor, Pool};
use ferris_shared::transfer::BoardInfo;

/// TODO: fetch image from logo and replace the path with the BASE64 String
#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub boards: Vec<BoardInfo>,
    pub workers: usize,
    pub name: String,
    pub logo: String,
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
}

#[derive(Clone)]
pub struct RuntimeConfig {
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
    pub name: String,
    pub logo: String,
    pub boards: Vec<BoardInfo>,
}

impl From<ServerConfig> for RuntimeConfig {
    fn from(config: ServerConfig) -> Self {
        Self {
            prevent_anonymous_posts: config.prevent_anonymous_posts,
            block_registrations: config.block_registrations,
            name: config.name,
            logo: config.logo,
            boards: config.boards,
        }
    }
}
