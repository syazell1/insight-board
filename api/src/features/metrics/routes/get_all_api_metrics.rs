use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{app_state::AppState, errors::AppError, features::auth::jwt::AuthUser};

#[tracing::instrument(skip_all)]
pub async fn get_all_api_metrics(
    auth: AuthUser,
    State(app_state): State<Arc<AppState>>,
) -> Result<Response, AppError> {
    let result = get_all_api_metrics_by_user_id(auth.0, &app_state.pool).await?;

    Ok((StatusCode::OK, Json(result)).into_response())
}

#[derive(Serialize, Deserialize)]
struct ApiMetricsData {
    id: Uuid,
    api_id: Uuid,
    name: String,
    url: String,
    checked_at: Option<DateTime<Utc>>,
    status_code: Option<i32>,
    latency_ms: Option<i32>,
    is_success: Option<bool>,
    error_message: Option<String>,
}
async fn get_all_api_metrics_by_user_id(
    user_id: Uuid,
    pool: &PgPool,
) -> Result<Vec<ApiMetricsData>, AppError> {
    let result = sqlx::query_as!(
        ApiMetricsData,
        r#"
            SELECT api_m.*, api_e.name, api_e.url
            FROM api_metrics api_m
            INNER JOIN api_endpoints api_e
            ON api_e.id = api_m.api_id
            WHERE api_e.user_id = $1
            ORDER BY api_m.checked_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
