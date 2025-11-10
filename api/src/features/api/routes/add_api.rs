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
};

#[tracing::instrument(skip_all)]
pub async fn add_api_endpoint(
    user: AuthUser,
    State(app_state): State<Arc<AppState>>,
    Json(data): Json<ApiFormData>,
) -> Result<Response, AppError> {
    data.validate()?;

    add_api(&data, user.0, &app_state.pool).await?;

    Ok((StatusCode::CREATED).into_response())
}

async fn add_api(data: &ApiFormData, user_id: Uuid, pool: &PgPool) -> Result<(), AppError> {
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

    Ok(())
}
