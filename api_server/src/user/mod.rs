pub mod dto;
pub mod handlers;
pub mod service;

use crate::utils::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user/signup", post(handlers::signup))
        .route("/user/signin", post(handlers::signin))
        .route("/user/signout", post(handlers::signout))
        .route("/user/refresh", post(handlers::refresh))
        .route("/user/verify", get(handlers::verify_email_get).post(handlers::verify_email_post))
        .route("/user/resend-verification", post(handlers::resend_verification))
        .route("/user/info", get(handlers::retrieve_user_info))
        .route("/user/stats", get(handlers::retrieve_stats))
        .route("/user", get(handlers::retrieve_me))
}
