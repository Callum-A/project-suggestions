use axum::{extract::State, http::StatusCode, response::Response, Extension, Json};

use crate::{
    model::user::User,
    state::AppState,
    util::{build_response, jwt::JWTClaims},
};

pub async fn get_profile(
    State(state): State<AppState>,
    Extension(user): Extension<JWTClaims>,
) -> Response {
    let user = state.user_repo.find_by_id(user.user_id).await;
    match user {
        Some(user) => build_response(StatusCode::OK, Json::<User>(user.into())),
        None => build_response(StatusCode::NOT_FOUND, "Not Found"),
    }
}
