pub mod dto;
pub mod handlers;
pub mod service;

use crate::utils::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/contest",
            get(handlers::retrieve)
                .post(handlers::create)
                .put(handlers::update)
                .delete(handlers::delete),
        )
        .route("/contests", get(handlers::retrieve_many))
        .route(
            "/contest/problems",
            get(handlers::retrieve_problems)
                .post(handlers::add_problems)
                .delete(handlers::delete_problem),
        )
        .route(
            "/contest/registration",
            post(handlers::register).delete(handlers::unregister),
        )
        .route("/contest/leaderboard", get(handlers::retrieve_leaderboard))
        .route("/contest/leaderboard/stream", get(handlers::stream_leaderboard))
        .route("/contest/leaderboard/ws", get(handlers::ws_leaderboard))
}
