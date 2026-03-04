use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Main application error type
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Authentication failed: {0}")]
    AuthError(String),

    #[error("Authorization failed: {0}")]
    Forbidden(String),

    #[error("Resource not found: {resource}")]
    NotFound { resource: String },

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error: {0}")]
    InternalServer(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Hashing error: {0}")]
    HashError(#[from] bcrypt::BcryptError),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Error response structure
#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "DATABASE_ERROR",
                format!("Database operation failed: {}", e),
            ),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, "AUTH_ERROR", msg.clone()),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, "FORBIDDEN", msg.clone()),
            AppError::NotFound { resource } => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                format!("{} not found", resource),
            ),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg.clone()),
            AppError::InternalServer(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg.clone(),
            ),
            AppError::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "SERVICE_UNAVAILABLE",
                msg.clone(),
            ),
            AppError::HashError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "HASH_ERROR",
                format!("Hashing failed: {}", e),
            ),
            AppError::JwtError(e) => (
                StatusCode::UNAUTHORIZED,
                "JWT_ERROR",
                format!("Token error: {}", e),
            ),
            AppError::Other(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                format!("Internal error: {}", e),
            ),
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
            details: None,
        };

        (status, Json(error_response)).into_response()
    }
}

/// Type alias for Result with AppError
pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    /// Create a not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound {
            resource: resource.into(),
        }
    }

    /// Create an authentication error
    pub fn auth(message: impl Into<String>) -> Self {
        Self::AuthError(message.into())
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    /// Create a conflict error
    pub fn conflict(message: impl Into<String>) -> Self {
        Self::Conflict(message.into())
    }

    /// Create a bad request error
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    /// Create a forbidden error
    pub fn forbidden(message: impl Into<String>) -> Self {
        Self::Forbidden(message.into())
    }

    /// Create an internal server error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::InternalServer(message.into())
    }

    /// Create a service unavailable error
    pub fn service_unavailable(message: impl Into<String>) -> Self {
        Self::ServiceUnavailable(message.into())
    }
}
