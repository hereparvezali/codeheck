pub mod create_submission;
pub mod dto;
pub mod retrieve_submission;
pub mod retrieve_submissions;
pub mod update_submission;

use crate::utils::app_state::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/submission",
            get(retrieve_submission::retrieve)
                .post(create_submission::create)
                .put(update_submission::update),
        )
        .route("/submissions", get(retrieve_submissions::retrieve))
}
