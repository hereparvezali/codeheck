use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MyErr {
    InternalServerError,
    DuplicateUsername,
    DuplicateEmail,
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

impl IntoResponse for MyErr {
    fn into_response(self) -> axum::response::Response {
        match self {
            MyErr::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            MyErr::DuplicateUsername => {
                (StatusCode::CONFLICT, "Duplicate Username").into_response()
            }
            MyErr::DuplicateEmail => (StatusCode::CONFLICT, "Duplicate Email").into_response(),
            MyErr::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            MyErr::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            MyErr::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg).into_response(),
            MyErr::Forbidden(msg) => (StatusCode::FORBIDDEN, msg).into_response(),
            MyErr::Conflict(msg) => (StatusCode::CONFLICT, msg).into_response(),
            MyErr::UnprocessableEntity(msg) => {
                (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
            }
            MyErr::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg).into_response(),
            MyErr::InternalServerErrorWithMessage(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            MyErr::ServiceUnavailable(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, msg).into_response()
            }
            MyErr::GatewayTimeout(msg) => (StatusCode::GATEWAY_TIMEOUT, msg).into_response(),
            MyErr::BadGateway(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
            MyErr::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, msg).into_response(),
            MyErr::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserPayload {
    pub email: String,
    pub username: String,
    pub password: String,
}



#[derive(Serialize, Deserialize, Debug)]
pub enum EmailOrUsername {
    Email(String),
    Username(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserPayload {
    pub email_or_username: EmailOrUsername,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    id: i64,
    username: String,
    email: String,
    access_token: String,
}
impl LoginUserResponse {
    pub fn new(
        id: i64,
        username: String,
        email: String,
        access_token: String,
    ) -> LoginUserResponse {
        Self {
            id,
            username,
            email,
            access_token,
        }
    }
}
