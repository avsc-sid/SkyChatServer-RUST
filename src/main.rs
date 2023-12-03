mod error;
mod routes;
mod structs;
mod macros;

use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use routes::*;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{services::{ServeDir, ServeFile}, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::structs::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
    .with(
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            // axum logs rejections from built-in extractors with the `axum::rejection`
            // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
            "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
        }),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

    dotenv().ok();
    let config = Arc::new(Config::init().await);

    // the following line exposes partial files, but what is the end user going to do anyways
    let static_files_service = ServeDir::new(&config.static_path)
        .append_index_html_on_directories(true)
        .call_fallback_on_method_not_allowed(true)
        .not_found_service(ServeFile::new(format!("{}/404.html", &config.static_path)));

    let app = Router::new()
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(CookieManagerLayer::new())
        )
        .route("/", get(root))
        .route("/auth", get(auth_page).post(auth_handler))
        .fallback_service(static_files_service)
        .with_state(config.clone())
        ;

    tracing::debug!("listening on {}", &config.address);

    axum::Server::bind(&config.address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
