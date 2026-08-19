use crate::utils::authenticator::authorizer;
use crate::{contest, problem, submission, user, utils::app_state::AppState};
use axum::{
    Router, middleware,
    routing::{get, post},
};


pub fn api_routes(state: &AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/user/signup", post(user::handlers::signup))
        .route("/user/signin", post(user::handlers::signin))
        .route("/user/signout", post(user::handlers::signout).get(user::handlers::signout))
        .route("/user/refresh", post(user::handlers::refresh).get(user::handlers::refresh));

    let protected_routes = Router::new()
        .merge(problem::router())
        .merge(contest::router())
        .merge(submission::router())
        .route("/user/info", get(user::handlers::retrieve_user_info))
        .route("/user/stats", get(user::handlers::retrieve_stats))
        .route("/user", get(user::handlers::retrieve_me))
        .layer(middleware::from_fn_with_state(state.clone(), authorizer));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}
