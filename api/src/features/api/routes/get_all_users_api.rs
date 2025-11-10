use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::AppError,
    features::{api::models::ApiEndpointData, auth::jwt::AuthUser},
};

// TODO: Add Pagination
#[tracing::instrument(skip_all)]
pub async fn get_all_users_api(
    auth: AuthUser,
    State(app_state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let result = get_all_users_api_endpoints_by_user_id(auth.0, &app_state.pool).await?;

    Ok((StatusCode::OK, Json(result)).into_response())
}

async fn get_all_users_api_endpoints_by_user_id(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<ApiEndpointData>, AppError> {
    let result = sqlx::query_as!(
        ApiEndpointData,
        r#"
            SELECT id, name, url, interval_seconds, is_active, created_at
            FROM api_endpoints
            WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
