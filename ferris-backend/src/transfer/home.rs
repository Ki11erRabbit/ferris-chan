
use serde::{Deserialize, Serialize};
use crate::transfer::BoardInfo;

#[derive(Deserialize, Serialize)]
pub struct HomePage {
    pub title: String,
    pub logo: String,
    pub boards: Vec<BoardInfo>,
}

impl Default for HomePage {
    fn default() -> Self {
        HomePage {
            title: String::from("Ferris-chan"),
            logo: String::new(),
            boards: vec![
                BoardInfo {
                    name: String::from("Technology"),
                    category: String::from("Interests"),
                }
            ],
        }
    }
}

