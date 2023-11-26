mod routes;
mod structs;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use routes::*;
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

use crate::structs::Config;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Arc::new(Config::init().await);
    let static_files_service = ServeDir::new(&config.static_path)
        .append_index_html_on_directories(true)
        .call_fallback_on_method_not_allowed(false)
        .not_found_service(ServeFile::new(format!("{}/404.html", &config.static_path)));

    let app = Router::new()
        .route("/", get(root))
        .route("/auth", post(auth_handler))
        .fallback_service(static_files_service)
        .with_state(config.clone());

    axum::Server::bind(&config.run)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
