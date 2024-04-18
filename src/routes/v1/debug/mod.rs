use axum::{Extension, Json};
use reqwest::StatusCode;

use crate::util::jwt::JWTClaims;

pub async fn hello_world_v1() -> &'static str {
    "Hello v1 API"
}

pub async fn whoami(Extension(user): Extension<JWTClaims>) -> (StatusCode, Json<JWTClaims>) {
    tracing::info!("Event=WhoAmI user={:?}", user);
    (StatusCode::OK, Json(user))
}
