pub mod auth_router;
pub mod dto;
pub mod refresh_access_token;
pub mod retrieve_user;
pub mod retrieve_user_contests;
pub mod retrieve_user_info;
pub mod retrieve_user_problems;
pub mod retrieve_user_solved_problems;
pub mod signin;
pub mod signout;
pub mod signup;

use axum::Router;
use crate::utils::app_state::AppState;

/// Returns the protected user routes (requires authentication)
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/user/retrieve_solved", axum::routing::get(retrieve_user_solved_problems::retrieve))
        .route("/user/retrieve_contests", axum::routing::get(retrieve_user_contests::retrieve))
        .route("/user/retrieve_problems", axum::routing::get(retrieve_user_problems::retrieve))
        .route("/user/retrieve", axum::routing::get(retrieve_user::retrieve))
        .route("/user/retrieve_user", axum::routing::get(retrieve_user_info::retrieve))
}

/// Returns the public auth routes (no authentication required)
pub fn auth_router() -> Router<AppState> {
    auth_router::routes()
}
