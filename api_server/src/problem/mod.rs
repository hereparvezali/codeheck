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
            "/problem",
            get(handlers::retrieve)
                .post(handlers::create)
                .put(handlers::update)
                .delete(handlers::delete),
        )
        .route("/problems", get(handlers::retrieve_many))
        .route(
            "/problem/testcases",
            get(handlers::retrieve_testcases)
                .post(handlers::create_testcases)
                .delete(handlers::delete_testcases),
        )
}
