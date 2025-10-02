use super::{refresh_access_token, signin, signout, signup};
use axum::{Router, routing::{get, post}};
use crate::utils::app_state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/user/signin", post(signin::signin))
        .route("/user/signup", post(signup::signup))
        .route("/user/signout", get(signout::signout))
        .route("/user/refresh", get(refresh_access_token::refresh))
}
