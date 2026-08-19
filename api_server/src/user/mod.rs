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
        .route("/user/info", get(handlers::retrieve_user_info))
        .route("/user/stats", get(handlers::retrieve_stats))
        .route("/user", get(handlers::retrieve_me))
}
