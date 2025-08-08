// use axum::{http::StatusCode, response::IntoResponse};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub enum MyErr {
//     InternalServerError,
//     BadRequest(String),
//     NotFound(String),
//     Unauthorized(String),
//     Forbidden(String),
//     Conflict(String),
//     UnprocessableEntity(String),
//     TooManyRequests(String),
//     InternalServerErrorWithMessage(String),
//     ServiceUnavailable(String),
//     GatewayTimeout(String),
//     BadGateway(String),
//     NotImplemented(String),
//     Unknown(String),
// }

// impl IntoResponse for MyErr {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             MyErr::InternalServerError => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
//             }

//             MyErr::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
//             MyErr::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
//             MyErr::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg).into_response(),
//             MyErr::Forbidden(msg) => (StatusCode::FORBIDDEN, msg).into_response(),
//             MyErr::Conflict(msg) => (StatusCode::CONFLICT, msg).into_response(),
//             MyErr::UnprocessableEntity(msg) => {
//                 (StatusCode::UNPROCESSABLE_ENTITY, msg).into_response()
//             }
//             MyErr::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg).into_response(),
//             MyErr::InternalServerErrorWithMessage(msg) => {
//                 (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
//             }
//             MyErr::ServiceUnavailable(msg) => {
//                 (StatusCode::SERVICE_UNAVAILABLE, msg).into_response()
//             }
//             MyErr::GatewayTimeout(msg) => (StatusCode::GATEWAY_TIMEOUT, msg).into_response(),
//             MyErr::BadGateway(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
//             MyErr::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, msg).into_response(),
//             MyErr::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
//         }
//     }
// }

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            MyErr::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            MyErr::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            MyErr::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            MyErr::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            MyErr::Forbidden(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            MyErr::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            MyErr::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            MyErr::TooManyRequests(msg) => (StatusCode::TOO_MANY_REQUESTS, msg.clone()),
            MyErr::InternalServerErrorWithMessage(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg.clone())
            }
            MyErr::ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg.clone()),
            MyErr::GatewayTimeout(msg) => (StatusCode::GATEWAY_TIMEOUT, msg.clone()),
            MyErr::BadGateway(msg) => (StatusCode::BAD_GATEWAY, msg.clone()),
            MyErr::NotImplemented(msg) => (StatusCode::NOT_IMPLEMENTED, msg.clone()),
            MyErr::Unknown(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = json!({
            "error": message,
            "code": status.as_u16(),
        });

        (status, axum::Json(body)).into_response()
    }
}
