use crate::{contest, problem, submission, user, utils::app_state::AppState};
use axum::Router;

/// Creates and returns all API routes
pub fn api_routes(state: &AppState) -> Router<AppState> {
    Router::new()
        .merge(protected_routes(state))
        .merge(public_routes())
}

/// Protected routes that require authentication
fn protected_routes(state: &AppState) -> Router<AppState> {
    use crate::utils::authenticator::authorizer;
    use axum::middleware;

    Router::new()
        .merge(submission::router())
        .merge(contest::router())
        .merge(problem::router())
        .merge(user::router())
        .layer(middleware::from_fn_with_state(state.clone(), authorizer))
}
/// Public routes that don't require authentication
fn public_routes() -> Router<AppState> {
    Router::new().merge(user::public_router())
}
