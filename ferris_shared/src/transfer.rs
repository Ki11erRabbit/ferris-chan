use serde::{Deserialize, Serialize};
use crate::transfer::home::HomePage;

pub mod home;
pub mod admin;
pub mod post;
pub mod user;

#[derive(Deserialize, Serialize, Clone)]
pub struct RootGetResponse {
    pub title: String,
    pub logo: String,
    pub boards: Vec<BoardInfo>
}

impl RootGetResponse {
    pub fn new(title: String, logo: String, boards: Vec<BoardInfo>) -> Self {
        Self {
            title,
            logo,
            boards,
        }
    }
}

impl Default for RootGetResponse {
    fn default() -> Self {
        RootGetResponse {
            title: String::from("Ferris-chan"),
            logo: String::new(),
            boards: vec![
                BoardInfo {
                    name: String::from("Technology"),
                    category: String::from("Interests")
                }
            ]
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct BoardInfo {
    pub name: String,
    pub category: String,
}