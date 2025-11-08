use chrono::Local;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::errors::AppError;

pub async fn add_users_token_by_token(
    token: &str,
    user_id: Uuid,
    executor: impl PgExecutor<'_>,
) -> Result<(), AppError> {
    let id = Uuid::now_v7();
    let date = Local::now();

    sqlx::query!(
        r#"
            INSERT INTO users_tokens (id, token, user_id, created_at)
            VALUES
            ($1, $2, $3, $4)
        "#,
        id,
        token,
        user_id,
        date
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn get_user_tokens_by_token(
    token: &str,
    executor: impl PgExecutor<'_>,
) -> Result<Option<Uuid>, AppError> {
    let user_id = sqlx::query_scalar!(
        r#"
            SELECT user_id FROM users_tokens WHERE token = $1
        "#,
        token,
    )
    .fetch_optional(executor)
    .await?;

    Ok(user_id)
}

pub async fn delete_refresh_token_by_token(
    token: &str,
    executor: impl PgExecutor<'_>,
) -> Result<(), AppError> {
    delete_token_by_token(token, executor).await
}

pub async fn delete_users_token_by_token(
    token: &str,
    executor: impl PgExecutor<'_>,
) -> Result<(), AppError> {
    delete_token_by_token(token, executor).await
}

pub async fn delete_all_refresh_token_by_user_id(
    user_id: Uuid,
    executor: impl PgExecutor<'_>,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            DELETE FROM users_tokens WHERE user_id = $1
        "#,
        user_id
    )
    .execute(executor)
    .await?;

    Ok(())
}

async fn delete_token_by_token(
    token: &str,
    executor: impl PgExecutor<'_>,
) -> Result<(), AppError> {
    sqlx::query!(
        r#"
            DELETE FROM users_tokens WHERE token = $1
        "#,
        token
    )
    .execute(executor)
    .await?;

    Ok(())
}
