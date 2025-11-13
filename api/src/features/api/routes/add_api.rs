use std::sync::Arc;

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    errors::AppError,
    features::{api::models::ApiFormData, auth::jwt::AuthUser},
    workers::api_monitoring::execute,
};

#[tracing::instrument(skip_all)]
pub async fn add_api_endpoint(
    user: AuthUser,
    State(app_state): State<Arc<AppState>>,
    Json(data): Json<ApiFormData>,
) -> Result<Response, AppError> {
    data.validate()?;

    let id = add_api(&data, user.0, &app_state.pool).await?;

    // Spawn the background monitor asynchronously, outside any lock
    let pool = app_state.pool.clone();
    let url = data.url.clone();
    let interval = data.interval_secs.unwrap_or(60);

    // Fire-and-forget task registration
    tokio::spawn(async move {
        let handle = execute(id, url, interval, &pool);
        let mut tasks = app_state.api_metrics_tasks.lock().await;
        tasks.insert(id, handle);
    });

    Ok((StatusCode::CREATED).into_response())
}

async fn add_api(data: &ApiFormData, user_id: Uuid, pool: &PgPool) -> Result<Uuid, AppError> {
    let id = Uuid::now_v7();

    sqlx::query!(
        r#"
            INSERT INTO api_endpoints (id, user_id, name, url, interval_seconds, is_active)
            VALUES
            ($1, $2, $3, $4, $5, $6)
        "#,
        id,
        user_id,
        data.name,
        data.url,
        data.interval_secs,
        data.is_active
    )
    .execute(pool)
    .await?;

    Ok(id)
}
