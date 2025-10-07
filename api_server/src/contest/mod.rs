pub mod add_contest_problem;
pub mod create_contest;
pub mod create_registration;
pub mod delete_registration;
pub mod dto;
pub mod retrieve_contest;
pub mod retrieve_contest_problems;
pub mod retrieve_contests;
pub mod retrieve_leaderboard;

use crate::utils::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/contest/registration",
            post(create_registration::create).delete(delete_registration::delete),
        )
        .route("/contests", get(retrieve_contests::retrieve))
        .route("/contest/add_problem", get(add_contest_problem::add))
        .route(
            "/contest/problems",
            get(retrieve_contest_problems::retrieve),
        )
        .route(
            "/contest",
            get(retrieve_contest::retrieve).post(create_contest::create),
        )
        .route("/contest/leaderboard", get(retrieve_leaderboard::retrieve))
}
