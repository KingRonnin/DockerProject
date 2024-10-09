use actix_web::{web, App, HttpServer};
use crate::handlers::user_handler::{create_student, get_students};
use crate::config::Config;

mod config;
mod models;
mod handlers;
mod db;
mod errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env().expect("Failed to load config")
    HttpServer::new(|| {
        App::new()
            .route("/student", web::post().to(create_student))
            .route("/student", web::post().to(get_students))
    })
    .bind(config.server_address)?
    .run()
    .await
}

