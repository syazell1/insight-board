use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::{TypedHeader, headers};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::{
    app_state::AppState,
    errors::AppError,
    features::auth::{
        jwt::generate_jwt_tokens,
        models::LoginFormData,
        pwd_hasher::{PwdHasher, ServerPwdHasher},
        repository::{
            add_users_token_by_token, delete_all_refresh_token_by_user_id,
            delete_users_token_by_token, get_user_tokens_by_token,
        },
        routes::{build_auth_response, create_refresh_token_cookie},
    },
};

#[tracing::instrument(skip_all)]
pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    TypedHeader(cookie): TypedHeader<headers::Cookie>,
    Json(input): Json<LoginFormData>,
) -> Result<Response, AppError> {
    input.validate()?;

    let pwd_hasher = ServerPwdHasher;
    let user_id = verify_user_credentials(input, &app_state.pool, pwd_hasher).await?;

    cleanup_old_refresh_token(&cookie, user_id, &app_state.pool).await?;

    let (access_token, refresh_token) = generate_jwt_tokens(user_id, &app_state.jwt_settings)?;

    let refresh_token_cookie = create_refresh_token_cookie(&refresh_token);
    add_users_token_by_token(refresh_token_cookie.value(), user_id, &app_state.pool).await?;

    Ok(build_auth_response(user_id, &access_token, refresh_token_cookie).into_response())
}

/// Verifies user credentials and returns the user ID if valid.
async fn verify_user_credentials(
    data: LoginFormData,
    pool: &PgPool,
    pwd_hasher: impl PwdHasher,
) -> Result<Uuid, AppError> {
    let user = sqlx::query_as!(
        UserAuth,
        r#"SELECT u.id, u.password
        FROM users u
        WHERE username = $1"#,
        data.username
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::InvalidCredentialError(
        "Invalid username or password".into(),
    ))?;

    pwd_hasher
        .verify_password(data.password, user.password)
        .await?;

    Ok(user.id)
}

/// Cleans up old refresh tokens. If a valid refresh token exists in cookies,
/// it deletes that specific token. Otherwise, it deletes all tokens for the user
/// to prevent token accumulation.
async fn cleanup_old_refresh_token(
    cookie: &headers::Cookie,
    user_id: Uuid,
    pool: &PgPool,
) -> Result<(), AppError> {
    if let Some(old_token) = cookie.get("rt") {
        // Try to delete the specific token
        let token_exists = get_user_tokens_by_token(old_token, pool).await?.is_some();

        if token_exists {
            delete_users_token_by_token(old_token, pool).await?;
        } else {
            // Token doesn't exist in DB (invalid/expired), clean up all tokens for this user
            delete_all_refresh_token_by_user_id(user_id, pool).await?;
        }
    } else {
        // No existing token cookie, clean up all tokens to prevent accumulation
        delete_all_refresh_token_by_user_id(user_id, pool).await?;
    }

    Ok(())
}

#[derive(sqlx::FromRow)]
struct UserAuth {
    id: Uuid,
    password: String,
}
