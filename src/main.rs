pub mod config;
pub mod model;
pub mod repositories;
pub mod routes;
pub mod state;
pub mod util;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use state::AppState;

use crate::{
    config::Config,
    repositories::{project::ProjectRepository, user::UserRepository},
    routes::v1::{
        debug::hello_world_v1,
        oauth::{google_authorize, google_callback},
        project::{
            create_project, delete_project_by_public_id, get_project_by_public_id, get_projects,
            update_project_by_public_id,
        },
    },
};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let config = Config::from_env().unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();
    let user_repo = UserRepository::new(pool.clone());
    let project_repo = ProjectRepository::new(pool.clone());
    let app_state = AppState {
        config,
        user_repo,
        project_repo,
    };

    let project_v1_router = Router::new()
        .route("/", post(create_project))
        .route("/", get(get_projects))
        .route("/:public_id", get(get_project_by_public_id))
        .route("/:public_id", delete(delete_project_by_public_id))
        .route("/:public_id", put(update_project_by_public_id));
    let api_v1_router = Router::new()
        .nest("/project", project_v1_router)
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
