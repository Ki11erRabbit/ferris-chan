use serde::{Deserialize, Serialize};

pub mod home;
pub mod admin;
pub mod post;
pub(crate) mod user;

#[derive(Deserialize, Serialize, Clone)]
pub struct RootGetResponse {
    title: String,
    logo: String,
    boards: Vec<BoardInfo>
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