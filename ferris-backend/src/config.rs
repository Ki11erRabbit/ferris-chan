use std::io::Read;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::Deserialize;
use sqlx::{Database, Executor};
use ferris_shared::transfer::BoardInfo;

/// TODO: fetch image from logo and replace the path with the BASE64 String
#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub board: BoardConfig,
    pub db: DbConfig,
}

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub workers: usize,
    #[serde(default = 1_000_000)]
    pub max_image_size: usize,
}

#[derive(Deserialize, Clone)]
pub struct DbConfig {
    pub url: String,
    pub r#type: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone)]
pub struct BoardConfig {
    pub boards: Vec<BoardInfo>,
    pub categories: Vec<String>,
    pub name: String,
    pub logo: String,
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
}

impl Config {

    /// Converts a path in logo into a base64 image
    /// This should only ever be called once
    pub fn get_logo(&mut self) {
        let mut buf = Vec::new();
        let Ok(mut file) = std::fs::File::open(&self.board.logo) else {
            return;
        };
        let Ok(_) = file.read_to_end(&mut buf) else {
            return;
        };

        let converted = BASE64_STANDARD.encode(&buf);
        self.board.logo = converted;

    }

}

impl From<Config> for BoardConfig {
    fn from(config: Config) -> Self {
        config.board
    }
}
