use axum::{response::IntoResponse, http::StatusCode};

#[tracing::instrument(name = "Health Checker")]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK).into_response()
}