pub mod create_problem;
pub mod create_testcases;
pub mod dto;
pub mod retrieve_problem;
pub mod retrieve_problems;
pub mod update_ispublic;

use crate::utils::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/problem",
            get(retrieve_problem::retrieve).post(create_problem::create),
        )
        .route("/problems", get(retrieve_problems::retrieve))
        .route("/problem/testcases", post(create_testcases::create))
}
