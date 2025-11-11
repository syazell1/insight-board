use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::AppError,
    features::{api::repository::get_api_endpoint_by_id, auth::jwt::AuthUser},
};

#[tracing::instrument(skip_all)]
pub async fn delete_api_endpoint(
    auth: AuthUser,
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    let api_id = Uuid::try_parse(&id)
        .with_context(|| AppError::BadRequestError("Invalid Uuid".to_string()))?;

    let api_data = get_api_endpoint_by_id(api_id, auth.0, &app_state.pool)
        .await?
        .ok_or(AppError::DataNotFoundError(
            "API data was not found".to_string(),
        ))?;

    delete_api_endpoint_by_id(api_data.id, auth.0, &app_state.pool).await?;

    Ok((StatusCode::OK).into_response())
}

async fn delete_api_endpoint_by_id(
    api_id: Uuid,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            DELETE FROM api_endpoints WHERE id = $1 AND user_id = $2
        "#,
        api_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
