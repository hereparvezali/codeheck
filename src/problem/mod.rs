pub mod create_problem;
pub mod create_testcases;
pub mod dto;
pub mod retrieve_problem;
pub mod retrieve_problems;
pub mod update_ispublic;

use axum::{
    Router,
    routing::{get, post},
};
use crate::utils::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/problem/create_testcases", post(create_testcases::create))
        .route("/problem/retrieve_problems", get(retrieve_problems::retrieve))
        .route("/problem/retrieve", get(retrieve_problem::retrieve))
        .route("/problem/create", post(create_problem::create))
}
