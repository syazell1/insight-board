use std::sync::Arc;

use axum::{
    Json, Router,
    http::header::SET_COOKIE,
    http::{HeaderName, StatusCode},
    response::AppendHeaders,
    routing::{get, post},
};
use cookie::time::Duration;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    features::auth::{
        models::AuthResponse,
        routes::{
            current_user::get_current_user, login::login_user, refresh_token::refresh_user_token,
            register::register_user,
        },
    },
};

mod current_user;
mod login;
mod refresh_token;
mod register;

/// Creates a secure HTTP-only cookie for the refresh token.
pub fn create_refresh_token_cookie(token: &str) -> cookie::Cookie<'_> {
    cookie::CookieBuilder::new("rt", token)
        .secure(true)
        .path("/")
        .max_age(Duration::days(7))
        .http_only(true)
        .same_site(cookie::SameSite::Lax)
        .build()
}

/// Builds the authentication response with the access token and refresh token cookie.
pub fn build_auth_response<'a>(
    user_id: Uuid,
    access_token: &'a str,
    refresh_token_cookie: cookie::Cookie<'_>,
) -> (
    StatusCode,
    AppendHeaders<[(HeaderName, String); 1]>,
    Json<AuthResponse<'a>>,
) {
    (
        StatusCode::OK,
        AppendHeaders([(SET_COOKIE, refresh_token_cookie.to_string())]),
        Json(AuthResponse {
            id: user_id,
            access_token,
        }),
    )
}

pub fn auth_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login_user))
        .route("/register", post(register_user))
        .route("/current-user", get(get_current_user))
        .route("/refresh", get(refresh_user_token))
}
