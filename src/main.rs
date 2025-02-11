mod controller;
mod repository;
mod model;
mod service;

use std::env;
use controller::controller::{
    get_task,
    get_all_task
};

use actix_web::{HttpServer, App, middleware::Logger, web};
use dotenv::dotenv;
use sqlx::PgPool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(pool.clone()))
            .service(get_task)
            .service(get_all_task)
    })
        .bind(("127.0.0.1",80))?
        .run()
        .await
}
