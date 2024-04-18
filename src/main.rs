pub mod config;
pub mod middleware;
pub mod model;
pub mod repositories;
pub mod routes;
pub mod state;
pub mod util;

use axum::{
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use state::AppState;

use crate::{
    config::Config,
    repositories::{project::ProjectRepository, tag::TagRepository, user::UserRepository},
    routes::v1::{
        debug::{hello_world_v1, whoami},
        oauth::{google_authorize, google_callback},
        project::{
            create_project, delete_project_by_public_id, get_project_by_public_id, get_projects,
            update_project_by_public_id,
        },
    },
    util::jwt::JWTClient,
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
    let tag_repo = TagRepository::new(pool.clone());
    let jwt_client = JWTClient::new(&config.jwt_secret);
    let app_state = AppState {
        config,
        user_repo,
        project_repo,
        tag_repo,
        jwt_client,
    };

    // Project routes
    let project_v1_router_protected = Router::new()
        .route("/", post(create_project))
        .route("/:public_id", delete(delete_project_by_public_id))
        .route("/:public_id", put(update_project_by_public_id))
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            middleware::authenticate,
        ));
    let project_v1_router = Router::new()
        .route("/", get(get_projects))
        .route("/:public_id", get(get_project_by_public_id))
        .nest("/", project_v1_router_protected);

    // Debug routes
    let debug_v1_router_protected =
        Router::new()
            .route("/whoami", get(whoami))
            .layer(axum::middleware::from_fn_with_state(
                app_state.clone(),
                middleware::authenticate,
            ));
    let debug_v1_router = Router::new()
        .route("/", get(hello_world_v1))
        .nest("/", debug_v1_router_protected);

    // v1 API routes
    let api_v1_router = Router::new()
        .nest("/project", project_v1_router)
        .nest("/debug", debug_v1_router)
        .route("/oauth/authorize/google", get(google_authorize))
        .route("/oauth/callback/google", get(google_callback));

    // Build the full app
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

async fn root() -> Response {
    let react_dev_server = std::env::var("REACT_DEV_SERVER").unwrap_or("".to_string());
    if react_dev_server.is_empty() {
        // TODO: Serve index.html
        return "Hello, World!".into_response();
    }

    // Redirect to React dev server
    axum::http::Response::builder()
        .status(axum::http::StatusCode::TEMPORARY_REDIRECT)
        .header("Location", react_dev_server)
        .body(axum::body::Body::empty())
        .unwrap()
}
