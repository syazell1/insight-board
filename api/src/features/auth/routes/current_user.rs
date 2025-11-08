use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app_state::AppState, errors::AppError, features::auth::jwt::AuthUser};

#[derive(Serialize)]
struct CurrentUserResponse {
    id: Uuid,
    username: String,
}

#[tracing::instrument(skip_all)]
pub async fn get_current_user(
    auth: AuthUser,
    State(app_state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let data = get_user_by_id(auth.0, &app_state.pool).await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

async fn get_user_by_id(user_id: Uuid, pool: &PgPool) -> Result<CurrentUserResponse, AppError> {
    let result = sqlx::query_as!(
        CurrentUserResponse,
        r#"
            SELECT id, username FROM users WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match result {
        Some(data) => Ok(data),
        None => {
            return Err(AppError::UnauthorizedError(
                "User was not found. Please login again.".into(),
            ));
        }
    }
}
