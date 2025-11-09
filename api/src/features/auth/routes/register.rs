use std::sync::Arc;

use anyhow::Context;
use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    config::JwtSettings,
    errors::AppError,
    features::auth::{
        jwt::encode_jwt, models::RegisterFormData, pwd_hasher::{PwdHasher, ServerPwdHasher}, repository::add_users_token_by_token, routes::{build_auth_response, create_refresh_token_cookie}
    },
};

#[tracing::instrument(skip_all)]
pub async fn register_user(
    State(app_state): State<Arc<AppState>>,
    Json(data): Json<RegisterFormData>,
) -> Result<Response, AppError> {
    data.validate()?;

    find_user_by_username(&data.username, &app_state.pool).await?;

    let user_id = create_user(&app_state.pool, &data).await?;

    let (access_token, refresh_token) = generate_jwt_tokens(user_id, &app_state.jwt_settings)?;

    let refresh_token_cookie = create_refresh_token_cookie(&refresh_token);
    add_users_token_by_token(refresh_token_cookie.value(), user_id, &app_state.pool).await?;
    let rt_cookie = create_refresh_token_cookie(&refresh_token);

    Ok(build_auth_response(user_id, &access_token, rt_cookie).into_response())
}

async fn find_user_by_username(username: &str, pool: &PgPool) -> Result<(), AppError> {
    let res = sqlx::query!(
        r#"
            SELECT id FROM users WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(_) = res {
        return Err(AppError::UnexpectedError(anyhow::anyhow!(
            "Username already taken"
        )));
    }

    Ok(())
}

async fn create_user(pool: &PgPool, data: &RegisterFormData) -> Result<Uuid, AppError> {
    let id = uuid::Uuid::now_v7();
    let date = chrono::Local::now();
    let pwd_hasher = ServerPwdHasher;

    let hashed_pwd = pwd_hasher.hash_password(data.password.to_string()).await?;

    sqlx::query!(
        r#"
            INSERT INTO users (id, username, password, created_at) VALUES ($1, $2, $3, $4)
        "#,
        id,
        data.username,
        hashed_pwd,
        date
    )
    .execute(pool)
    .await?;

    Ok(id)
}

pub fn generate_jwt_tokens(
    user_id: Uuid,
    jwt_settings: &JwtSettings,
) -> Result<(String, String), AppError> {
    let at = encode_jwt(user_id, jwt_settings, false).context("Failed to generate jwt token.")?;
    let rt = encode_jwt(user_id, jwt_settings, true).context("Failed to generate jwt token.")?;

    Ok((at, rt))
}
