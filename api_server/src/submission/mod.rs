pub mod consumer;
pub mod dto;
pub mod handlers;
pub mod service;

use crate::utils::app_state::AppState;
use axum::{
    Router,
    routing::get,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/submission",
            get(handlers::retrieve)
                .post(handlers::create)
                .put(handlers::update),
        )
        .route("/submissions", get(handlers::retrieve_many))
        .route("/submission/stream", get(handlers::stream_submission))
        .route("/submission/ws", get(handlers::ws_submission))
}
