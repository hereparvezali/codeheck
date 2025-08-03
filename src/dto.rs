use axum::{http::StatusCode, response::IntoResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MyErr {
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

impl IntoResponse for MyErr {
    fn into_response(self) -> axum::response::Response {
        match self {
            MyErr::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }

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
    pub created_at: Option<NaiveDateTime>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProblemPayload {
    pub title: String,
    pub slug: String,
    pub statement: Option<String>,
    pub input_spec: Option<String>,
    pub output_spec: Option<String>,
    pub sample_inputs: Option<String>,
    pub time_limit: i16,
    pub memory_limit: i16,
    pub difficulty: Option<String>,
    pub author_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveProblemQuery {
    pub id: Option<i64>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateContestPayload {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_public: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveUserinfoQuery {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
}
