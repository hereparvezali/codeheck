pub mod dto;
pub mod refresh;
pub mod retrieve_user;
pub mod retrieve_user_info;
pub mod signin;
pub mod signout;
pub mod signup;

use crate::utils::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

/// Returns the protected user routes (requires authentication)
pub fn router() -> Router<AppState> {
    Router::new().route("/user", get(retrieve_user::retrieve))
}

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/user/signin", post(signin::signin))
        .route("/user/signup", post(signup::signup))
        .route("/user/signout", get(signout::signout))
        .route("/user/refresh", get(refresh::refresh))
}
