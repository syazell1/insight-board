use std::sync::Arc;

use anyhow::Context;
use axum::{
    extract::{FromRef, FromRequestParts},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    app_state::AppState, config::JwtSettings, errors::AppError
};

#[derive(Deserialize, Serialize, Clone)]
pub struct Claims {
    pub id: Uuid,
    pub aud: String,
    pub iss: String,
    pub exp: usize,
}

pub fn encode_jwt(
    user_id: Uuid,
    jwt_settings: &JwtSettings,
    is_refresh_token: bool,
) -> Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        id: user_id,
        aud: jwt_settings.audience.to_string(),
        iss: jwt_settings.issuer.to_string(),
        exp: if is_refresh_token {
            (Utc::now() + Duration::try_weeks(1).unwrap()).timestamp() as usize
        } else {
            (Utc::now() + Duration::try_minutes(15).unwrap()).timestamp() as usize
        },
    };

    let secret_key = if is_refresh_token {
        EncodingKey::from_secret(jwt_settings.refresh_token_secret.expose_secret().as_bytes())
    } else {
        EncodingKey::from_secret(jwt_settings.access_token_secret.expose_secret().as_bytes())
    };

    encode(&Header::default(), &claims, &secret_key)
}

pub fn decode_jwt(
    token: &str,
    jwt_settings: &JwtSettings,
    is_refresh_token: bool,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_issuer(&[jwt_settings.issuer.to_string()]);
    validation.set_audience(&[jwt_settings.audience.to_string()]);

    let secret_key = if is_refresh_token {
        DecodingKey::from_secret(jwt_settings.refresh_token_secret.expose_secret().as_bytes())
    } else {
        DecodingKey::from_secret(jwt_settings.access_token_secret.expose_secret().as_bytes())
    };

    decode(token, &secret_key, &validation)
}

pub fn generate_jwt_tokens(
    user_id: Uuid,
    jwt_settings: &JwtSettings,
) -> Result<(String, String), AppError> {
    let at = encode_jwt(user_id, jwt_settings, false).context("Failed to generate jwt token.")?;
    let rt = encode_jwt(user_id, jwt_settings, true).context("Failed to generate jwt token.")?;

    Ok((at, rt))
}

#[derive(Deserialize, Serialize)]
pub struct AuthUser(pub Uuid);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|e| AppError::InvalidJwtTokenError(e.to_string()))?;

        let app_state = Arc::from_ref(state);

        let token_data = decode_jwt(bearer.token(), &app_state.jwt_settings, false)
            .map_err(|e| AppError::InvalidJwtTokenError(e.to_string()))?;

        Ok(Self(token_data.claims.id))
    }
}
