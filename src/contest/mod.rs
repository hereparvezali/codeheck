pub mod add_contest_problem;
pub mod create_contest;
pub mod create_register;
pub mod delete_register;
pub mod dto;
pub mod retrieve_contest;
pub mod retrieve_contest_problems;
pub mod retrieve_contests;

use axum::{
    Router,
    routing::{delete, get, post},
};
use crate::utils::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/contest/delete_register/{id}", delete(delete_register::delete))
        .route("/contest/create_register/{contest_id}", post(create_register::create))
        .route("/contest/retrieve_contests", get(retrieve_contests::retrieve))
        .route("/contest/add_problem", get(add_contest_problem::add))
        .route("/contest/retrieve_problems", get(retrieve_contest_problems::retrieve))
        .route("/contest/retrieve", get(retrieve_contest::retrieve))
        .route("/contest/create", post(create_contest::create))
}
