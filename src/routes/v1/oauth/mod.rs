use crate::{model::user::AuthProvider, state::AppState, util::jwt::JWTClaims};
use axum::{
    extract::{Query, State},
    response::Response,
};
use axum_extra::extract::cookie::{Cookie, SameSite};

pub async fn google_authorize(State(state): State<AppState>) -> Response {
    let query_string = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", &state.config.google_client_id)
        .append_pair("redirect_uri", &state.config.google_redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", &state.config.google_scopes.join(" "))
        .append_pair("state", "1") // TODO: generate a random state and store it in a cookie to retrieve and check on redirect
        .finish();

    let url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?{}",
        query_string
    );
    tracing::info!("redirecting to {}", url);
    axum::http::Response::builder()
        .status(axum::http::StatusCode::TEMPORARY_REDIRECT)
        .header("Location", url)
        .body(axum::body::Body::empty())
        .unwrap()
}

#[derive(serde::Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
    pub state: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GoogleTokenResponse {
    pub access_token: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct GoogleUserInfoResponse {
    pub email: String,
    pub given_name: String,
}

pub async fn google_callback(
    State(state): State<AppState>,
    Query(query): Query<GoogleCallbackQuery>,
) -> Response {
    let code = query.code;

    let response = reqwest::Client::new()
        .post(&state.config.google_token_url)
        .form(&[
            ("code", code),
            ("client_id", state.config.google_client_id),
            ("client_secret", state.config.google_client_secret),
            ("redirect_uri", state.config.google_redirect_uri),
            ("grant_type", "authorization_code".to_string()),
        ])
        .send()
        .await
        .unwrap();

    let response = response.json::<GoogleTokenResponse>().await.unwrap();
    let token = response.access_token;

    let response = reqwest::Client::new()
        .get(&state.config.google_user_info_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Accept", "application/json")
        .send()
        .await
        .unwrap();
    let response = response.json::<GoogleUserInfoResponse>().await.unwrap();

    tracing::info!("email={} name={}", response.email, response.given_name);

    let potential_user = state.user_repo.find_by_email(&response.email).await;
    let user = match potential_user {
        Some(user) => {
            // User exists
            tracing::info!("user exists: {:?}", user);
            user
        }
        None => {
            // User does not exist
            tracing::info!("user does not exist, creating user");
            state
                .user_repo
                .create(
                    &uuid::Uuid::new_v4().to_string(),
                    &response.email,
                    &response.given_name,
                    AuthProvider::Google,
                )
                .await
        }
    };

    let token = state.jwt_client.encode(JWTClaims::from(&user));
    let cookie = Cookie::build(("token", token))
        .path("/")
        .max_age(time::Duration::days(1))
        .same_site(SameSite::Lax)
        .http_only(false)
        .build();

    // Redirect to index page
    axum::http::Response::builder()
        .status(axum::http::StatusCode::TEMPORARY_REDIRECT)
        .header("Location", "/")
        .header(axum::http::header::SET_COOKIE, cookie.to_string())
        .body(axum::body::Body::empty())
        .unwrap()
}
