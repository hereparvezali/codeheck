use crate::error::AppError;
use chrono::Utc;
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

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST / 2)
        .map_err(|e| AppError::internal(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(hashed_password: &str, password: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hashed_password)
        .map_err(|e| AppError::internal(format!("Failed to verify password: {}", e)))
}
