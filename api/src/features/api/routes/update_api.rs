use std::sync::Arc;

use anyhow::Context;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sqlx::{Executor, Postgres, Transaction};
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    errors::AppError,
    features::{api::models::ApiFormData, auth::jwt::AuthUser},
};

#[tracing::instrument(skip_all)]
pub async fn update_api_endpoint(
    user: AuthUser,
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(data): Json<ApiFormData>,
) -> Result<Response, AppError> {
    let id = Uuid::try_parse(&id)
        .with_context(|| AppError::BadRequestError("Invalid UUID Format".into()))?;

    data.validate()?;

    let mut tx = app_state.pool.begin().await?;

    check_api_by_id(id, &mut tx).await?;

    update_api_endpoint_by_id(id, user.0, &data, &mut tx).await?;

    Ok((StatusCode::OK).into_response())
}

async fn check_api_by_id(api_id: Uuid, tx: &mut Transaction<'_, Postgres>) -> Result<(), AppError> {
    let query = sqlx::query_scalar!(r#"SELECT id FROM api_endpoints WHERE id = $1"#, api_id);

    match tx.fetch_optional(query).await? {
        Some(_) => Ok(()),
        None => Err(AppError::DataNotFoundError("API Data was not found".into())),
    }
}

async fn update_api_endpoint_by_id(
    api_id: Uuid,
    user_id: Uuid,
    data: &ApiFormData,
    tx: &mut Transaction<'_, Postgres>,
) -> Result<(), AppError> {
    let query = sqlx::query!(
        r#"
            UPDATE api_endpoints SET url = $1, name = $2, interval_seconds = $3, is_active = $4
            WHERE id = $5 AND user_id = $6
             "#,
        data.url,
        data.name,
        data.interval_secs,
        data.is_active,
        api_id,
        user_id
    );

    tx.execute(query).await?;

    Ok(())
}
