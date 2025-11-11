use crate::errors::AppError;
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
#[cfg(test)]
use mockall::{automock, predicate::*};
use password_hash::{PasswordHasher, SaltString};
use rand_core::OsRng;

pub struct ServerPwdHasher;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait PwdHasher {
    async fn verify_password(
        &self,
        password: String,
        encrypted_password: String,
    ) -> Result<(), AppError>;
    async fn hash_password(&self, password: String) -> Result<String, AppError>;
}

#[async_trait]
impl PwdHasher for ServerPwdHasher {
    async fn verify_password(
        &self,
        password: String,
        encrypted_password: String,
    ) -> Result<(), AppError> {
        tokio::task::spawn_blocking(move || verify_pwd(password, encrypted_password))
            .await
            .context("Failed to spawn blocking task for password verification")?
    }

    async fn hash_password(&self, password: String) -> Result<String, AppError> {
        tokio::task::spawn_blocking(move || hash_pwd(password))
            .await
            .context("Failed to spawn blocking task for password hashing")?
    }
}

/// Creates an Argon2 hasher instance with consistent parameters.
/// Uses Argon2id algorithm with recommended parameters for password hashing.
fn create_argon2_hasher() -> Result<Argon2<'static>, AppError> {
    let params = argon2::Params::new(15000, 2, 1, None).map_err(|e| {
        AppError::UnexpectedError(anyhow::anyhow!("Invalid Argon2 parameters: {}", e))
    })?;

    Ok(Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        params,
    ))
}

fn hash_pwd(password: String) -> Result<String, AppError> {
    let hasher = create_argon2_hasher()?;
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = hasher
        .hash_password(password.as_bytes(), &salt)
        .context("Failed to hash password")?;

    Ok(password_hash.to_string())
}

fn verify_pwd(password: String, encrypted_password: String) -> Result<(), AppError> {
    let password_hash = PasswordHash::new(&encrypted_password).map_err(|e| {
        AppError::InvalidCredentialError(format!("Invalid password hash format: {}", e))
    })?;

    let hasher = create_argon2_hasher()?;

    hasher
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|e| {
            AppError::InvalidCredentialError(format!("Password verification failed: {}", e))
        })
}
