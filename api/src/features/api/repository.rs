use sqlx::PgExecutor;
use uuid::Uuid;

use crate::{errors::AppError, features::api::models::ApiEndpointData};

#[tracing::instrument(skip_all)]
pub async fn get_api_endpoint_by_id(
    api_id: Uuid,
    user_id: Uuid,
    pool: impl PgExecutor<'_>,
) -> Result<Option<ApiEndpointData>, AppError> {
    let result = sqlx::query_as!(
        ApiEndpointData,
        r#"
            SELECT id, name, url, interval_seconds, is_active, created_at
            FROM api_endpoints
            WHERE id = $1 AND user_id = $2
        "#,
        api_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;
    Ok(result)
}
