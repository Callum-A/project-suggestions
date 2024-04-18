use axum::{
    http::status::StatusCode,
    response::{IntoResponse, Response},
};

pub fn build_response(status: StatusCode, body: impl IntoResponse) -> Response {
    let mut response = body.into_response();
    *response.status_mut() = status;
    response
}
