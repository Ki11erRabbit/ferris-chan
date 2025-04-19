mod endpoints;
mod transfer;
mod constants;
mod config;
mod database;

use std::fs::OpenOptions;
use std::io::Read;
use actix_web::{middleware, App, HttpServer};
use actix_web::web::Data;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use crate::config::ServerConfig;

#[derive(Clone)]
pub struct AppState {
    pub config: ServerConfig,
    pub db: SqlitePool,
}
impl AppState {
    pub fn new(config: ServerConfig, db: SqlitePool) -> Self {
        Self { config, db }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let mut config = OpenOptions::new()
        .read(true)
        .open("config.toml")?;

    let mut buf = String::new();
    config.read_to_string(&mut buf)?;


    let config: ServerConfig = toml::from_str(buf.as_str()).expect("Failed to parse config.toml");
    let port = config.port;

    let options = SqliteConnectOptions::new()
        .filename("database.sqlite")
        .create_if_missing(true);

    let mut pool =  SqlitePool::connect_with(options).await.unwrap();

    database::initialize_database(&config, &mut pool).await
        .expect("Failed to initialize database");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(AppState::new(config.clone(), pool.clone())))
            .service(endpoints::get_home)
            .service(endpoints::user::login_user)
            .service(endpoints::user::logout_user)
            .service(endpoints::admin::admin_remove_post)
            .service(endpoints::post::get_posts)
            .service(endpoints::post::get_post_replies)
            .service(endpoints::post::create_post)
            .service(endpoints::post::create_post_reply)
    })
        .bind(("127.0.0.1", port))?
        .run()
        .await?;


    Ok(())
}
