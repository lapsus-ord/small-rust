use axum::{routing::get, Json, Router};
use serde::Serialize;
use small_rust_core::{emojis::get_random_emoji, hostname::get_hostname};
use tower_http::cors::CorsLayer;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub async fn init_server() {
    pretty_env_logger::init_timed();

    let app = Router::new()
        .route("/hello", get(hello_handler))
        .layer(CorsLayer::very_permissive());

    let app_host = std::env::var("APP_HOST").unwrap_or("0.0.0.0".to_owned());
    let app_port = std::env::var("APP_PORT").unwrap_or("3000".to_owned());
    let server_addr = format!("{app_host}:{app_port}");
    info!("Server starting on {server_addr}");

    axum::Server::bind(&server_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server")
}

#[derive(Debug, Serialize)]
struct Hello {
    message: String,
    hostname: String,
}

async fn hello_handler() -> Result<Json<Hello>, String> {
    let hostname = get_hostname()?;

    let hello_res = Hello {
        message: format!("alive! {}", get_random_emoji().unwrap_or("")),
        hostname,
    };

    Ok(Json(hello_res))
}
