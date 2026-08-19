use crate::{entity::users, error::AppError, utils::config::Config};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

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

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::internal(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(hashed_password: &str, password: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hashed_password)
        .map_err(|e| AppError::internal(format!("Failed to verify password: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hash_and_verify() {
        let password = "my_secure_password_123!";
        let hash = hash_password(password).expect("Hashing should succeed");

        assert!(verify_password(&hash, password).expect("Verification should succeed"));
        assert!(!verify_password(&hash, "wrong_password").expect("Verification should succeed"));
    }
}

