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
        jwt::{decode_jwt, generate_jwt_tokens},
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

    let user_token_data = match get_user_tokens_by_token(rt, &app_state.pool).await? {
        Some(data) => data,
        None => {
            let token_data = decode_jwt(rt, &app_state.jwt_settings, true)
                .map_err(|e| AppError::UnauthorizedError(e.to_string()))?;

            let user = get_user_info_by_id(token_data.claims.id, &app_state.pool).await?;

            delete_all_refresh_token_by_user_id(user.id, &app_state.pool).await?;
            return Err(AppError::UnauthorizedError(
                "Refresh token reuse found.".into(),
            ));
        }
    };

    let token_data = match decode_jwt(rt, &app_state.jwt_settings, false) {
        Ok(data) => data,
        Err(e) => {
            if *e.kind() == jsonwebtoken::errors::ErrorKind::ExpiredSignature {
                delete_refresh_token_by_token(rt, &app_state.pool).await?;
            }

            return Err(AppError::UnauthorizedError(e.to_string()));
        }
    };

    if user_token_data != token_data.claims.id {
        return Err(AppError::UnauthorizedError("Invalid jwt token".into()));
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
