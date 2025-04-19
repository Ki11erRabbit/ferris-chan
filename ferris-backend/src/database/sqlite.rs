use sqlx::{Row, SqliteConnection, SqlitePool};
use uuid::{Timestamp, Uuid, timestamp::context::Context};
use crate::config::ServerConfig;
use crate::transfer::BoardInfo;
use crate::transfer::post::Post;

pub async fn initialize_database(config: &ServerConfig, pool: &mut SqlitePool) -> sqlx::Result<()> {
    let mut connection = pool.begin().await?;

    let statements = [
        "CREATE TABLE IF NOT EXISTS Board (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
        "CREATE TABLE IF NOT EXISTS Category (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT)",
        "CREATE TABLE IF NOT EXISTS User (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT, password TEXT, email TEXT)",
        "CREATE TABLE IF NOT EXISTS AuthToken (id INTEGER PRIMARY KEY AUTOINCREMENT, token TEXT, timestamp BIG INT, user_id INTEGER, FOREIGN KEY (user_id) REFERENCES User(id))",
        "CREATE TABLE IF NOT EXISTS Admin (id INTEGER PRIMARY KEY AUTOINCREMENT, user_id INTEGER, FOREIGN KEY (user_id) REFERENCES User(id))",
        "CREATE TABLE IF NOT EXISTS Post (id INTEGER, board_id INTEGER, category_id INTEGER, image TEXT, text TEXT, user_id INTEGER, timestamp BIG INT, parent INTEGER, FOREIGN KEY (user_id) REFERENCES User(id), FOREIGN KEY (board_id) REFERENCES Board(id), FOREIGN KEY (category_id) REFERENCES Category(id), FOREIGN KEY(parent) REFERENCES Post(id), PRIMARY KEY (id))",
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


    for board in &config.boards {
        add_board(&mut *connection, board).await?
    }

    connection.commit().await
}

async fn add_board<'a>(connection: &mut SqliteConnection, board: &BoardInfo) -> sqlx::Result<()> {
    let BoardInfo { name ,category } = board;

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

pub async fn login_user(pool: &SqlitePool, email: &str, password: &str) -> sqlx::Result<String> {
    let mut connection = pool.begin().await?;

    // TODO: HASH THE PASSWORD!!!!

    let id = sqlx::query("SELECT id FROM User WHERE email = $1 AND password = $2")
        .bind(email)
        .bind(password)
        .fetch_one(&mut *connection)
        .await?;

    let id = id.get::<i64, &str>("id");

    let token = Uuid::new_v4();
    let token = token.to_string();

    let timestamp = Timestamp::now(Context::new_random());
    let (timestamp, _) = timestamp.to_unix();

    sqlx::query("INSERT INTO AuthToken (token, user_id, timestamp) VALUES ($1, $2, $3)")
        .bind(&token)
        .bind(id)
        .bind(timestamp as i64)
        .execute(&mut *connection)
        .await?;

    connection.commit().await?;

    Ok(token)
}

pub async fn logout_user(pool: &SqlitePool, token: &str) -> sqlx::Result<()> {
    let mut connection = pool.begin().await?;

    sqlx::query("DELETE FROM AuthToken WHERE token = $1")
        .bind(&token)
        .execute(&mut *connection)
        .await?;

    connection.commit().await?;
    Ok(())
}

pub async fn get_posts(pool: &SqlitePool, board: &str, category: &str, count: i64, offset: i64) -> sqlx::Result<Vec<Post>> {
    let mut connection = pool.begin().await?;
    let mut output = Vec::new();

    log::info!("getting posts");

    let result = sqlx::query("SELECT Post.id as post_number, username, image, text, timestamp FROM Post JOIN User ON User.id = Post.user_id JOIN Board ON Post.board_id = Board.id JOIN Category ON Post.category_id = Category.id where Board.name = $1 AND Category.name = $2 ORDER BY Post.id DESC LIMIT $4 OFFSET $3")
        .bind(board)
        .bind(category)
        .bind(offset)
        .bind(count)
        .fetch_all(&mut *connection)
        .await?;

    log::info!("got data");

    for row in result {
        output.push(Post {
            username: row.get("username"),
            image: row.get("image"),
            text: row.get("text"),
            unix_timestamp: row.get::<i64, _>("timestamp") as usize,
            post_number: row.get::<i64, _>("post_number") as usize,
        })
    }

    connection.commit().await?;
    Ok(output)
}

pub async fn get_post_replies(pool: &SqlitePool, parent: i64, count: i64, offset: i64) -> sqlx::Result<Vec<Post>> {
    let mut connection = pool.begin().await?;
    let mut output = Vec::new();

    log::info!("getting posts");

    let result = sqlx::query("SELECT Post.id as post_number, username, image, text, timestamp FROM Post JOIN User ON User.id = Post.user_id JOIN Board ON Post.board_id = Board.id JOIN Category ON Post.category_id = Category.id where parent = $1 ORDER BY Post.id DESC LIMIT $3 OFFSET $2")
        .bind(parent)
        .bind(offset)
        .bind(count)
        .fetch_all(&mut *connection)
        .await?;

    log::info!("got data");

    for row in result {
        output.push(Post {
            username: row.get("username"),
            image: row.get("image"),
            text: row.get("text"),
            unix_timestamp: row.get::<i64, _>("timestamp") as usize,
            post_number: row.get::<i64, _>("post_number") as usize,
        })
    }

    connection.commit().await?;
    Ok(output)
}