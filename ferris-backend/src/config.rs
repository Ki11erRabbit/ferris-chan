use std::io::Read;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::Deserialize;
use sqlx::{Database, Executor};
use ferris_shared::transfer::BoardInfo;

/// TODO: fetch image from logo and replace the path with the BASE64 String
#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub boards: Vec<BoardInfo>,
    pub categories: Vec<String>,
    pub workers: usize,
    pub name: String,
    pub logo: String,
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
}

impl ServerConfig {

    /// Converts a path in logo into a base64 image
    /// This should only ever be called once
    pub fn get_logo(&mut self) {
        let mut buf = Vec::new();
        let Ok(mut file) = std::fs::File::open(&self.logo) else {
            return;
        };
        let Ok(_) = file.read_to_end(&mut buf) else {
            return;
        };

        let converted = BASE64_STANDARD.encode(&buf);
        self.logo = converted;

    }

}

#[derive(Clone)]
pub struct RuntimeConfig {
    pub prevent_anonymous_posts: bool,
    pub block_registrations: bool,
    pub name: String,
    pub logo: String,
    pub categories: Vec<String>,
    pub boards: Vec<BoardInfo>,
}

impl From<ServerConfig> for RuntimeConfig {
    fn from(config: ServerConfig) -> Self {
        Self {
            prevent_anonymous_posts: config.prevent_anonymous_posts,
            block_registrations: config.block_registrations,
            name: config.name,
            logo: config.logo,
            categories: config.categories,
            boards: config.boards,
        }
    }
}
