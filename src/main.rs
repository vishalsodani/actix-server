use crate::utils::redis_utils::connect_redis;
use actix_web::{App, HttpServer};
use deadpool_postgres::Config;
use dotenv;
use tokio_postgres::NoTls;

// json - postgres example
mod Json;

// user authentication
mod auth;

mod models;
mod types;
mod utils;

// private routes
mod private;

// middleware
mod middleware;

// errors example
mod errors;

// downloads
mod downloads;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // loading .env file
    dotenv::dotenv().ok();

    // creating postgres pool connection
    let cfg = Config::from_env("PG").unwrap();
    let pool = cfg.create_pool(NoTls).unwrap();
    pool.get().await.unwrap();

    // redis cache
    let redis_client = connect_redis();

    // actix server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(redis_client.clone())
            .service(auth::auth_routes())
            .service(Json::json_routes())
            .service(errors::register_error_handlers())
            .service(downloads::register_download_routes())
            .service(private::register_private().wrap(middleware::private::CheckToken))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
