pub mod create_submission;
pub mod dto;
pub mod retrieve_submission;
pub mod retrieve_submissions;
pub mod update_submission;

use axum::{
    Router,
    routing::{get, post, put},
};
use crate::utils::app_state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/submission/update", put(update_submission::update))
        .route("/submission/retrieve/{id}", get(retrieve_submission::retrieve))
        .route("/submissions/retrieve", get(retrieve_submissions::retrieve))
        .route("/submission/create", post(create_submission::create))
}
