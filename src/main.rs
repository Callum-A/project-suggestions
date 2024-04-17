pub mod config;
pub mod model;
pub mod routes;
pub mod state;

use axum::{routing::get, Router};
use state::AppState;

use crate::{
    config::Config,
    routes::v1::{
        debug::hello_world_v1,
        oauth::{google_authorize, google_callback},
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let config = Config::from_env().unwrap();
    let app_state = AppState { config };

    let api_v1_router = Router::new()
        .route("/", get(hello_world_v1))
        .route("/oauth/authorize/google", get(google_authorize))
        .route("/oauth/callback/google", get(google_callback));

    let app = Router::new()
        .nest("/api/v1", api_v1_router)
        .route("/", get(root))
        .with_state(app_state.clone());

    let listener = tokio::net::TcpListener::bind(&format!(
        "{}:{}",
        app_state.config.host, app_state.config.port
    ))
    .await
    .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
