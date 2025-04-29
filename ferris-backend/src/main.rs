mod endpoints;
mod constants;
mod config;
mod database;

use std::fs::OpenOptions;
use std::io::Read;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use actix_web::web::{Data, PayloadConfig};
use clap::Parser;
use crate::config::{BoardConfig, Config};
use crate::database::{DatabaseDriver, DbFactory};

#[derive(Clone)]
pub struct AppState {
    pub config: BoardConfig,
    pub db: Arc<dyn DatabaseDriver>,
    pub max_image_size: usize
}
impl AppState {
    pub fn new(config: BoardConfig, db: Arc<dyn DatabaseDriver>, max_image_size: usize) -> Self {
        Self { config, db, max_image_size }
    }
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "/etc/ferris-chan/config.toml")]
    config: String,
    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(&args.log_level));

    let mut config = OpenOptions::new()
        .read(true)
        .open(&args.config)?;

    let mut buf = String::new();
    config.read_to_string(&mut buf)?;


    let mut config: Config = toml::from_str(buf.as_str()).expect("Failed to parse config.toml");

    config.get_logo();

    let port = config.server.port;
    let max_image_size = config.server.max_image_size;

    let db_driver = DbFactory::initialize_database(&config).await?;

    let workers = config.server.workers;
    let config: BoardConfig = config.into();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .send_wildcard();

        let payload = PayloadConfig::new(max_image_size+ 1_000_000);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .app_data(Data::new(payload))
            .app_data(Data::new(AppState::new(config.clone(), db_driver.clone(), max_image_size)))
            .service(endpoints::get_home)
            .service(endpoints::user::register_user)
            .service(endpoints::user::login_user)
            .service(endpoints::user::logout_user)
            .service(endpoints::admin::admin_remove_post)
            .service(endpoints::post::get_posts)
            .service(endpoints::post::get_post_replies)
            .service(endpoints::post::get_post_image)
            .service(endpoints::post::create_post)
            .service(endpoints::post::create_post_reply)
    })
        .bind(("127.0.0.1", port))?
        .workers(workers)
        .run()
        .await?;

    Ok(())
}
