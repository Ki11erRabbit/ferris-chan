use serde::{Deserialize, Serialize};

pub mod home;
pub mod admin;
pub mod post;
pub(crate) mod user;

#[derive(Deserialize, Serialize)]
pub struct BoardInfo {
    pub name: String,
    pub category: String,
}