pub mod config;
pub mod state;

use axum::{
    extract::{Query, State},
    response::Response,
    routing::get,
    Router,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let config = config::Config::from_env().unwrap();
    let app_state = state::AppState { config };

    let app = Router::new()
        .route("/", get(root))
        .route("/authorize/google", get(google_authorize))
        .route("/callback/google", get(google_callback))
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

async fn google_authorize(State(state): State<AppState>) -> Response {
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
struct GoogleCallbackQuery {
    code: String,
    // state: String,
}

#[derive(serde::Deserialize, Debug)]
struct GoogleTokenResponse {
    access_token: String,
}

#[derive(serde::Deserialize, Debug)]
struct GoogleUserInfoResponse {
    email: String,
    given_name: String,
}

async fn google_callback(
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

    // TODO: Find or create user in database

    // TODO: Return a JWT token in a cookie for the client
    let token = format!("mock-jwt-token-{}", response.email);
    let cookie = Cookie::build(("token", token))
        .path("/")
        .max_age(time::Duration::days(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    // Redirect to index page
    axum::http::Response::builder()
        .status(axum::http::StatusCode::TEMPORARY_REDIRECT)
        .header("Location", "/")
        .header(axum::http::header::SET_COOKIE, cookie.to_string())
        .body(axum::body::Body::empty())
        .unwrap()
}

async fn root() -> &'static str {
    "Hello, World!"
}
