use std::path::PathBuf;
use anyhow::Error;
use async_trait::async_trait;
use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqliteConnection, SqlitePool};
use sqlx::sqlite::SqliteConnectOptions;
use uuid::Uuid;
use crate::config::Config;
use crate::constants;
use ferris_shared::transfer::BoardInfo;
use ferris_shared::transfer::post::Post;
use crate::constants::{get_base64_len, AUTH_TOKEN_LIFETIME};
use crate::database::{DatabaseDriver, DatabaseError};

fn create_sqlite_database_location() -> anyhow::Result<PathBuf> {
    let mut path = PathBuf::new();
    path.push("/var/ferris-chan");
    if !path.exists() {
        std::fs::create_dir_all(&path)?;
    }
    path.push("db.sqlite");
    Ok(path)
}

pub struct SqliteDB {
    pool: SqlitePool,
    max_image_size: usize,
}

impl SqliteDB {
    pub async fn new() -> anyhow::Result<Self> {

        let path = create_sqlite_database_location()?;

        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);

        let pool =  SqlitePool::connect_with(options).await?;
        Ok(Self { pool, max_image_size: 0 })
    }

    pub fn validate_image_size(&self, img_string: &str) -> Option<bool> {
        get_base64_len(img_string).map(|len| {
            log::info!("img_len = {}", len);
            len <= self.max_image_size
        })
    }
}

#[async_trait]
impl DatabaseDriver for SqliteDB {
    async fn initialize_database(&mut self, config: &Config) -> anyhow::Result<()> {
        self.max_image_size = config.server.max_image_size;

        let mut connection = self.pool.begin().await?;

        let statements = [
            "CREATE TABLE IF NOT EXISTS Board (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
            "CREATE TABLE IF NOT EXISTS Category (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
            "CREATE TABLE IF NOT EXISTS User (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT, password TEXT, email TEXT)",
            "CREATE TABLE IF NOT EXISTS AuthToken (id INTEGER PRIMARY KEY AUTOINCREMENT, token TEXT, timestamp BIG INT, user_id INTEGER, FOREIGN KEY (user_id) REFERENCES User(id))",
            "CREATE TABLE IF NOT EXISTS Admin (id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, FOREIGN KEY (user_id) REFERENCES User(id))",
            "CREATE TABLE IF NOT EXISTS Post (id INTEGER PRIMARY KEY AUTOINCREMENT, board_id INTEGER, category_id INTEGER, image TEXT, alt_text TEXT, text TEXT, user_id INTEGER, timestamp BIG INT, parent INTEGER, FOREIGN KEY (user_id) REFERENCES User(id), FOREIGN KEY (board_id) REFERENCES Board(id), FOREIGN KEY (category_id) REFERENCES Category(id), FOREIGN KEY(parent) REFERENCES Post(id))",
        ];

        for stmt in statements {
            sqlx::query(stmt)
                .execute(&mut *connection)
                .await?;
        }

        sqlx::query("INSERT OR IGNORE INTO User (id, username, password, email) VALUES (1, $1, $2, $3)")
            .bind(String::from("Anonymous"))
            .bind(String::new())
            .bind(String::new())
            .execute(&mut *connection)
            .await?;


        for board in &config.board.boards {
            add_board(&mut *connection, board).await?
        }

        Ok(connection.commit().await?)
    }
    async fn register_user(&self, username: &str, email: &str, password: &str) -> anyhow::Result<String> {
        let mut connection = self.pool.begin().await?;

        let result = sqlx::query("SELECT 1 FROM User WHERE username=$1 OR email=$2")
            .bind(username)
            .bind(email)
            .fetch_optional(&mut *connection)
            .await?;

        if let Some(_) = result {
            return Err(Error::from(DatabaseError::UserAlreadyExists))
        }


        let password = constants::hash_password(password);

        sqlx::query("INSERT INTO User (username, password, email) VALUES ($1, $2, $3)")
            .bind(username)
            .bind(&password)
            .bind(email)
            .execute(&mut *connection)
            .await?;


        let id = sqlx::query("SELECT id FROM User WHERE email = $1 AND password = $2")
            .bind(email)
            .bind(&password)
            .fetch_one(&mut *connection)
            .await?;

        let id = id.get::<i64, &str>("id");

        let token = Uuid::new_v4();
        let token = token.to_string();

        let timestamp = Utc::now();
        let timestamp = timestamp.timestamp();

        sqlx::query("INSERT INTO AuthToken (token, user_id, timestamp) VALUES ($1, $2, $3)")
            .bind(&token)
            .bind(id)
            .bind(timestamp)
            .execute(&mut *connection)
            .await?;

        connection.commit().await?;

        Ok(token)
    }


    async fn login_user(&self, email: &str, password: &str) -> anyhow::Result<(String, bool)> {
        let mut connection = self.pool.begin().await?;

        let password = constants::hash_password(password);

        let id = sqlx::query("SELECT id FROM User WHERE email = $1 AND password = $2")
            .bind(email)
            .bind(&password)
            .fetch_optional(&mut *connection)
            .await?;

        let Some(id) = id else {
            return Err(Error::from(DatabaseError::UserOrPasswordDoesNotMatch))
        };

        let id = id.get::<i64, &str>("id");


        let is_admin = sqlx::query("SELECT id FROM Admin WHERE user_id=$1 LIMIT 1")
            .bind(id)
            .fetch_optional(&mut *connection)
            .await?;

        let is_admin = if is_admin.is_some() { true } else { false };

        let token = Uuid::new_v4();
        let token = token.to_string();

        let timestamp = Utc::now();
        let timestamp = timestamp.timestamp();

        sqlx::query("INSERT INTO AuthToken (token, user_id, timestamp) VALUES ($1, $2, $3)")
            .bind(&token)
            .bind(id)
            .bind(timestamp)
            .execute(&mut *connection)
            .await?;


        connection.commit().await?;

        Ok((token, is_admin))
    }

    async fn logout_user(&self, token: &str) -> anyhow::Result<()> {
        let mut connection = self.pool.begin().await?;

        sqlx::query("DELETE FROM AuthToken WHERE token = $1")
            .bind(&token)
            .execute(&mut *connection)
            .await?;

        connection.commit().await?;
        Ok(())
    }

    async fn get_posts(&self, board: &str, category: &str, count: i64, offset: i64) -> anyhow::Result<Vec<Post>> {
        let mut connection = self.pool.begin().await?;
        let mut output = Vec::new();


        let result = sqlx::query("SELECT Post.id as post_number, username, image, text, parent, timestamp, alt_text FROM Post JOIN User ON User.id = Post.user_id JOIN Board ON Post.board_id = Board.id JOIN Category ON Post.category_id = Category.id where Board.name = $1 AND Category.name = $2 AND Post.parent IS NULL ORDER BY Post.id DESC LIMIT $4 OFFSET $3")
            .bind(board)
            .bind(category)
            .bind(offset)
            .bind(count)
            .fetch_all(&mut *connection)
            .await?;


        for row in result {
            output.push(Post {
                username: row.get("username"),
                image: row.get("image"),
                alt_text: row.get("alt_text"),
                text: row.get("text"),
                timestamp: row.get::<i64, _>("timestamp"),
                post_number: row.get::<i64, _>("post_number") as usize,
                parent: row.get::<i64, _>("parent")
            })
        }

        connection.commit().await?;
        Ok(output)
    }

    async fn get_post_replies(&self, parent: i64, count: i64, offset: i64) -> anyhow::Result<Vec<Post>> {
        let mut connection = self.pool.begin().await?;
        let mut output = Vec::new();

        let result = sqlx::query("SELECT Post.id as post_number, username, image, text, parent, timestamp, alt_text FROM Post JOIN User ON User.id = Post.user_id JOIN Board ON Post.board_id = Board.id JOIN Category ON Post.category_id = Category.id where parent = $1 ORDER BY Post.id DESC LIMIT $3 OFFSET $2")
            .bind(parent)
            .bind(offset)
            .bind(count)
            .fetch_all(&mut *connection)
            .await?;


        for row in result {
            output.push(Post {
                username: row.get("username"),
                image: row.get("image"),
                alt_text: row.get("alt_text"),
                text: row.get("text"),
                timestamp: row.get::<i64, _>("timestamp"),
                post_number: row.get::<i64, _>("post_number") as usize,
                parent: row.get::<i64, _>("parent")
            })
        }

        connection.commit().await?;
        Ok(output)
    }

    async fn get_post_image(&self, post_id: i64) -> anyhow::Result<Vec<u8>> {
        let mut connection = self.pool.begin().await?;

        let base64_img = sqlx::query("SELECT id FROM Post WHERE id = $1")
            .bind(post_id)
            .fetch_all(&mut *connection)
            .await?;

        let string: String = base64_img[0].get("image");
        let bytes = BASE64_STANDARD.decode(string.as_bytes()).expect("Base64 Decode error");

        connection.commit().await?;

        Ok(bytes)
    }

    async fn create_post(&self, board: &str, category: &str, image: &str, alt_text: &str, text: &str, auth_token: Option<String>) -> anyhow::Result<Post> {
        let size = self.validate_image_size(image);
        if let Some(false) = size {
            return Err(Error::from(DatabaseError::ImageLargerThanPermitted))
        } else if size.is_none() {
            return Err(Error::from(DatabaseError::ImageIsInvalidBase64));
        }

        let mut connection = self.pool.begin().await?;

        let board_id = sqlx::query("SELECT id FROM Board WHERE name = $1")
            .bind(board)
            .fetch_all(&mut *connection)
            .await?;

        let board_id: i64 = board_id[0].get("id");

        let category_id = sqlx::query("SELECT id FROM Category WHERE name = $1")
            .bind(category)
            .fetch_all(&mut *connection)
            .await?;

        let category_id: i64 = category_id[0].get("id");

        let user_id: i64 = match auth_token {
            Some(token) => {
                let result = sqlx::query("SELECT user_id, timestamp FROM AuthToken WHERE token = $1")
                    .bind(token.as_str())
                    .fetch_optional(&mut *connection)
                    .await?;

                if let Some(result) = result {
                    let timestamp = result.get("timestamp");
                    let user_id = result.get("user_id");
                    match check_auth_token(timestamp) {
                        Err(DatabaseError::AuthTokenExpired) => {
                            invalidate_auth_token(&mut *connection, &token).await?;
                            connection.commit().await?;

                            return Err(Error::from(DatabaseError::AuthTokenExpired))
                        }
                        _ => {
                            user_id
                        }
                    }
                } else {
                    1
                }
            }
            None => 1, // use Anonymous user id
        };

        let timestamp = Utc::now();
        let timestamp = timestamp.timestamp();

        sqlx::query("INSERT INTO Post (board_id, category_id, user_id, image, text, timestamp, alt_text) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(board_id)
            .bind(category_id)
            .bind(user_id)
            .bind(image)
            .bind(text)
            .bind(timestamp)
            .bind(alt_text)
            .execute(&mut *connection)
            .await?;

        let result = sqlx::query("SELECT Post.id as post_number, username, image, text, parent, timestamp, alt_text FROM Post JOIN User ON User.id = Post.user_id JOIN Board ON Post.board_id = Board.id JOIN Category ON Post.category_id = Category.id where Board.name = $1 AND Category.name = $2 AND Post.timestamp = $3")
            .bind(board)
            .bind(category)
            .bind(timestamp)
            .fetch_all(&mut *connection)
            .await?;

        if let Some(row) = result.first() {
            let output = Post {
                username: row.get("username"),
                image: row.get("image"),
                alt_text: row.get("alt_text"),
                text: row.get("text"),
                timestamp: row.get::<i64, _>("timestamp"),
                post_number: row.get::<i64, _>("post_number") as usize,
                parent: row.get::<i64, _>("parent")
            };

            connection.commit().await?;

            return Ok(output);
        }
        connection.commit().await?;

        todo!("change this to return a generic error")
    }

    async fn create_post_reply(&self, board: &str, category: &str, image: &str, alt_text: &str, text: &str, parent: i64, auth_token: Option<String>) -> anyhow::Result<Post> {
        let size = self.validate_image_size(image);
        if let Some(false) = size {
            return Err(Error::from(DatabaseError::ImageLargerThanPermitted))
        } else if size.is_none() {
            return Err(Error::from(DatabaseError::ImageIsInvalidBase64));
        }


        let mut connection = self.pool.begin().await?;

        let board_id = sqlx::query("SELECT id FROM Board WHERE name = $1")
            .bind(board)
            .fetch_all(&mut *connection)
            .await?;

        let board_id: i64 = board_id[0].get("id");

        let category_id = sqlx::query("SELECT id FROM Category WHERE name = $1")
            .bind(category)
            .fetch_all(&mut *connection)
            .await?;

        let category_id: i64 = category_id[0].get("id");

        let user_id: i64 = match auth_token {
            Some(token) => {

                let result = sqlx::query("SELECT user_id, timestamp FROM AuthToken WHERE token = $1")
                    .bind(token.as_str())
                    .fetch_optional(&mut *connection)
                    .await?;

                if let Some(result) = result {
                    let timestamp = result.get("timestamp");
                    let user_id = result.get("user_id");
                    match check_auth_token(timestamp) {
                        Err(DatabaseError::AuthTokenExpired) => {
                            invalidate_auth_token(&mut *connection, &token).await?;
                            connection.commit().await?;

                            return Err(Error::from(DatabaseError::AuthTokenExpired))
                        }
                        _ => {
                            user_id
                        }
                    }
                } else {
                    1
                }
            }
            None => 1, // use Anonymous user id
        };

        let timestamp = Utc::now();
        let timestamp = timestamp.timestamp();

        sqlx::query("INSERT INTO Post (board_id, category_id, user_id, image, text, timestamp, parent, alt_text) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(board_id)
            .bind(category_id)
            .bind(user_id)
            .bind(image)
            .bind(text)
            .bind(timestamp)
            .bind(parent)
            .bind(alt_text)
            .execute(&mut *connection)
            .await?;

        let result = sqlx::query("SELECT Post.id as post_number, username, image, text, parent, timestamp FROM Post JOIN User ON User.id = Post.user_id JOIN Board ON Post.board_id = Board.id JOIN Category ON Post.category_id = Category.id where Board.name = $1 AND Category.name = $2 AND Post.timestamp = $3")
            .bind(board)
            .bind(category)
            .bind(timestamp)
            .fetch_all(&mut *connection)
            .await?;

        if let Some(row) = result.first() {
            let output = Post {
                username: row.get("username"),
                image: row.get("image"),
                alt_text: row.get("alt_text"),
                text: row.get("text"),
                timestamp: row.get::<i64, _>("timestamp"),
                post_number: row.get::<i64, _>("post_number") as usize,
                parent: row.get::<i64, _>("parent")
            };

            connection.commit().await?;

            return Ok(output);
        }
        connection.commit().await?;



        todo!("change this to return a generic error")
    }

    async fn delete_post(&self, post_id: i64, token: &str) -> anyhow::Result<()> {
        let mut connection = self.pool.begin().await?;

        let auth_check = sqlx::query("SELECT id FROM AuthToken WHERE token = $1")
            .bind(token)
            .fetch_optional(&mut *connection)
            .await?;

        if let Some(_) = auth_check {
            _ = sqlx::query("DELETE FROM Post WHERE id = $1")
                .bind(post_id)
                .execute(&mut *connection)
                .await?;
        }

        connection.commit().await?;
        Ok(())
    }
}

async fn add_board<'a>(connection: &mut SqliteConnection, board: &BoardInfo) -> anyhow::Result<()> {
    let BoardInfo { name, category } = board;

    let result = sqlx::query("SELECT id FROM Board WHERE name = $1")
        .bind(name)
        .fetch_all(&mut *connection)
        .await?;

    if result.is_empty() {
        sqlx::query("INSERT OR IGNORE INTO Board (name) VALUES ($1)")
            .bind(name)
            .execute(&mut *connection)
            .await?;
    }

    let result = sqlx::query("SELECT id, name FROM Category WHERE name = $1")
        .bind(category)
        .fetch_all(&mut *connection)
        .await?;

    if result.is_empty() {
        sqlx::query("INSERT OR IGNORE INTO Category (name) VALUES ($1)")
            .bind(category)
            .execute(&mut *connection)
            .await?;
    }

    Ok(())
}


fn check_auth_token(token_timestamp: i64) -> anyhow::Result<(), DatabaseError> {
    let timestamp = Utc::now();

    let token_timestamp = DateTime::<Utc>::from_timestamp(token_timestamp, 0).unwrap();

    let delta = token_timestamp.signed_duration_since(timestamp);
    if delta.num_days() as usize > AUTH_TOKEN_LIFETIME {
        Err(DatabaseError::AuthTokenExpired)
    } else {
        Ok(())
    }
}

async fn invalidate_auth_token(connection: &mut SqliteConnection, token: &str) -> anyhow::Result<()> {

    sqlx::query("DELETE FROM AuthToken WHERE token = $1")
        .bind(token)
        .execute(connection)
        .await?;

    Ok(())
}
