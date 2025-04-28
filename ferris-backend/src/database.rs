pub mod sqlite;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::sync::Arc;
use async_trait::async_trait;
use ferris_shared::transfer::post::Post;
use crate::config::Config;
use crate::database::sqlite::SqliteDB;

#[async_trait]
pub trait DatabaseDriver: Send + Sync {
    async fn initialize_database(&mut self, config: &Config) -> anyhow::Result<()>;

    async fn register_user(&self, username: &str, email: &str, password: &str) -> anyhow::Result<String>;

    async fn login_user(&self, email: &str, password: &str) -> anyhow::Result<(String, bool)>;

    async fn logout_user(&self, token: &str) -> anyhow::Result<()>;

    async fn get_posts(&self, board: &str, category: &str, count: i64, offset: i64) -> anyhow::Result<Vec<Post>>;

    async fn get_post_replies(&self, parent: i64, count: i64, offset: i64) -> anyhow::Result<Vec<Post>>;

    async fn get_post_image(&self, post_id: i64) -> anyhow::Result<Vec<u8>>;

    async fn create_post(&self, board: &str, category: &str, image: &str, alt_text: &str, text: &str, auth_token: Option<String>) -> anyhow::Result<Post>;

    async fn create_post_reply(&self, board: &str, category: &str, image: &str, alt_text: &str, text: &str, parent: i64, auth_token: Option<String>) -> anyhow::Result<Post>;

    async fn delete_post(&self, post_id: i64, token: &str) -> anyhow::Result<()>;
}


pub struct DbFactory;

impl DbFactory {
    pub async fn initialize_database(config: &Config) -> anyhow::Result<Arc<dyn DatabaseDriver>> {
        let mut driver = match config.db.r#type.as_str() {
            "sqlite" => Self::create_sqlite_database().await?,
            _ => anyhow::bail!("Unsupported database type: {}", config.db.r#type),
        };

        driver.initialize_database(config).await?;

        Ok(driver)
    }

    async fn create_sqlite_database() -> anyhow::Result<Arc<dyn DatabaseDriver>> {
        let driver = Arc::new(SqliteDB::new().await?);

        Ok(driver)
    }
}

#[derive(Debug)]
pub enum DatabaseError {
    AuthTokenExpired,
    UserAlreadyExists,
    UserOrPasswordDoesNotMatch,
    ImageLargerThanPermitted,
    ImageIsInvalidBase64
}


impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::AuthTokenExpired => {
                write!(f, "Auth token expired")
            }
            DatabaseError::UserAlreadyExists => {
                write!(f, "User already exists")
            }
            DatabaseError::UserOrPasswordDoesNotMatch => {
                write!(f, "User or password doesn't match")
            }
            DatabaseError::ImageLargerThanPermitted => {
                write!(f, "Image larger than permitted")
            }
            DatabaseError::ImageIsInvalidBase64 => {
                write!(f, "Image is invalid")
            }
        }
    }
}

impl Error for DatabaseError {}