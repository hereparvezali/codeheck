use crate::{entity::users, error::AppError, utils::config::Config};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tokio::task::spawn_blocking;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub exp: usize,
}

impl Claim {
    pub fn new(id: i64, username: String, email: String, minutes: usize) -> Self {
        Self {
            id,
            username,
            email,
            exp: Utc::now().timestamp() as usize + minutes * 60,
        }
    }
}

pub fn generate_access_token(user: &users::Model, config: &Config) -> Result<String, AppError> {
    let claim = Claim::new(
        user.id,
        user.username.clone(),
        user.email.clone(),
        config.jwt.access_token_expiry_minutes,
    );
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::internal(format!("Failed to encode JWT: {}", e)))
}

pub fn generate_refresh_token(user: &users::Model, config: &Config) -> Result<String, AppError> {
    let claim = Claim::new(
        user.id,
        user.username.clone(),
        user.email.clone(),
        config.jwt.refresh_token_expiry_minutes,
    );
    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::internal(format!("Failed to encode JWT: {}", e)))
}

pub fn generate_access_token_from_claim(
    claim: &Claim,
    config: &Config,
) -> Result<String, AppError> {
    let new_claim = Claim::new(
        claim.id,
        claim.username.clone(),
        claim.email.clone(),
        config.jwt.access_token_expiry_minutes,
    );
    encode(
        &Header::default(),
        &new_claim,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )
    .map_err(|e| AppError::internal(format!("Failed to encode JWT: {}", e)))
}

pub fn validate_token(token: &str, config: &Config) -> Result<Claim, AppError> {
    let data = decode::<Claim>(
        token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::auth("Invalid or expired token"))?;
    Ok(data.claims)
}

pub async fn hash_password(password: &str) -> Result<String, AppError> {
    let password = password.to_string();
    spawn_blocking(move || {
        bcrypt::hash(password, bcrypt::DEFAULT_COST / 2)
            .map_err(|e| AppError::internal(format!("Failed to hash password: {}", e)))
    })
    .await
    .map_err(|e| AppError::internal(e.to_string()))?
}

pub async fn verify_password(password: &str, hashed_password: &str) -> Result<bool, AppError> {
    let password = password.to_string();
    let hashed_password = hashed_password.to_string();
    spawn_blocking(move || {
        bcrypt::verify(password, &hashed_password)
            .map_err(|e| AppError::internal(format!("Failed to verify password: {}", e)))
    })
    .await
    .map_err(|e| AppError::internal(e.to_string()))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_password_hash_and_verify() {
        let password = "my_secure_password_123!";
        let hash = hash_password(password)
            .await
            .expect("Hashing should succeed");

        assert!(
            verify_password(password, &hash)
                .await
                .expect("Verification should succeed")
        );
        assert!(
            !verify_password("wrong_password", &hash)
                .await
                .expect("Verification should succeed")
        );
    }
}
