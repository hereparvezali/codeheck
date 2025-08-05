use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

use crate::{
    contest::{create_contest, retrieve_contest, retrieve_contest_problems},
    problem::{create_problem, retrieve_problem},
    submission::{create_submission, retrieve_submission},
    user::{login, signup},
    utils::{app_state::AppState, middlewares::authorizer},
};

pub async fn app() -> Router {
    let state = AppState::new().await;
    Router::new()
        .route("/submission/create", post(create_submission::create))
        .route(
            "/contest/retrieve_problems",
            get(retrieve_contest_problems::retrieve),
        )
        .route("/contest/retrieve", get(retrieve_contest::retrieve))
        .route("/contest/create", post(create_contest::create))
        .route("/problem/retrieve", get(retrieve_problem::retrieve))
        .route("/problem/create", post(create_problem::create))
        .layer(middleware::from_fn_with_state(state.clone(), authorizer))
        .route("/user/login", post(login::login))
        .route("/user/signup", post(signup::signup))
        .route(
            "/submission/retrieve/{id}",
            get(retrieve_submission::retrieve),
        )
        .fallback(async || "Hi! URL not found??")
        .layer(CorsLayer::very_permissive())
        .with_state(state)
}
