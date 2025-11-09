use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::{TypedHeader, headers::Cookie};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    errors::AppError,
    features::auth::{
        jwt::{decode_jwt, decode_jwt_with_options, generate_jwt_tokens},
        repository::{
            add_users_token_by_token, delete_all_refresh_token_by_user_id,
            delete_refresh_token_by_token, get_user_tokens_by_token,
        },
        routes::{build_auth_response, create_refresh_token_cookie},
    },
};

#[tracing::instrument(skip_all)]
pub async fn refresh_user_token(
    State(app_state): State<Arc<AppState>>,
    TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Response, AppError> {
    let rt = match cookie.get("rt") {
        Some(data) => data,
        None => {
            return Err(AppError::NoRefreshTokenError(
                "Refresh token was not found.".into(),
            ));
        }
    };

    // Check if the refresh token exists in the database
    let db_user_id = match get_user_tokens_by_token(rt, &app_state.pool).await? {
        Some(user_id) => user_id,
        None => {
            // Token not in DB - might be a reused token
            // Try to decode it (even if expired) to detect reuse
            if let Ok(token_data) = decode_jwt_with_options(rt, &app_state.jwt_settings, true, true) {
                // Token is valid (even if expired) - this is token reuse
                let user_id = token_data.claims.id;
                delete_all_refresh_token_by_user_id(user_id, &app_state.pool).await?;
                return Err(AppError::UnauthorizedError(
                    "Refresh token reuse detected. All tokens have been revoked.".into(),
                ));
            } else {
                // Token is invalid or malformed
                return Err(AppError::UnauthorizedError(
                    "Invalid refresh token.".into(),
                ));
            }
        }
    };

    // Token exists in DB - decode and validate it (check expiration)
    let token_data = match decode_jwt(rt, &app_state.jwt_settings, true) {
        Ok(data) => data,
        Err(e) => {
            // Token is expired or invalid
            if *e.kind() == jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                delete_refresh_token_by_token(rt, &app_state.pool).await?;
            }

            return Err(AppError::UnauthorizedError(e.to_string()));
        }
    };

    // Verify the user_id from database matches the user_id in token claims
    if db_user_id != token_data.claims.id {
        return Err(AppError::UnauthorizedError(
            "Token user ID mismatch.".into(),
        ));
    }

    let user = get_user_info_by_id(token_data.claims.id, &app_state.pool).await?;

    delete_refresh_token_by_token(rt, &app_state.pool).await?;

    let (at, rt) = generate_jwt_tokens(user.id, &app_state.jwt_settings)?;

    add_users_token_by_token(&rt, user.id, &app_state.pool).await?;

    let refresh_token_cookie = create_refresh_token_cookie(&rt);

    let res = build_auth_response(user.id, &at, refresh_token_cookie);

    Ok(res.into_response())
}

#[derive(Deserialize)]
struct UserInfo {
    id: Uuid,
}

#[tracing::instrument(skip_all)]
async fn get_user_info_by_id(user_id: Uuid, pool: &PgPool) -> Result<UserInfo, AppError> {
    let result = sqlx::query_as!(
        UserInfo,
        r#"
            SELECT id FROM users WHERE id = $1 
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    match result {
        Some(data) => Ok(data),
        None => {
            return Err(AppError::DataNotFoundError(format!(
                "User with Id '{}' was not found",
                user_id
            )));
        }
    }
}
