mod endpoints;
mod transfer;
mod constants;
mod config;
mod database;

use std::fs::OpenOptions;
use std::io::Read;
use actix_web::{App, HttpRequest, HttpServer};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{AnyPool, SqlitePool};
use crate::config::ServerConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut config = OpenOptions::new()
        .read(true)
        .open("config.toml")?;

    let mut buf = String::new();
    config.read_to_string(&mut buf)?;

    println!("{}", buf);

    let config: ServerConfig = toml::from_str(buf.as_str()).expect("Failed to parse config.toml");

    let options = SqliteConnectOptions::new()
        .filename("database.sqlite")
        .create_if_missing(true);

    let mut pool =  SqlitePool::connect_with(options).await.unwrap();

    database::initialize_database(&config, &mut pool).await
        .expect("Failed to initialize database");

    pool.close().await;

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(endpoints::get_home)
            .service(endpoints::user::login_user)
            .service(endpoints::user::logout_user)
            .service(endpoints::admin::admin_remove_post)
            .service(endpoints::post::get_posts)
            .service(endpoints::post::get_post_replies)
    })
        .bind(("127.0.0.1", config.port))?
        .run()
        .await?;


    Ok(())
}
