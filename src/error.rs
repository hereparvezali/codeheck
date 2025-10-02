use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum AppError {
    InternalServerError,
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    UnprocessableEntity(String),
    TooManyRequests(String),
    InternalServerErrorWithMessage(String),
    ServiceUnavailable(String),
    GatewayTimeout(String),
    BadGateway(String),
    NotImplemented(String),
    Unknown(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg).into_response(),
            AppError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg).into_response(),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg).into_response(),
            AppError::UnprocessableEntity(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
            }
            AppError::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg).into_response(),
            AppError::InternalServerErrorWithMessage(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            AppError::ServiceUnavailable(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, msg).into_response()
            }
            AppError::GatewayTimeout(msg) => (StatusCode::GATEWAY_TIMEOUT, msg).into_response(),
            AppError::BadGateway(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
            AppError::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, msg).into_response(),
            AppError::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

// Type alias for backward compatibility during migration
pub type MyErr = AppError;
