use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

use crate::{
    problem::{create_problem, retrieve_problem},
    user::{login, signup},
    utils::{app_state::AppState, middlewares::authorizer},
};

pub async fn app() -> Router {
    let state = AppState::new().await;
    Router::new()
        .route("/problem/retrieve", get(retrieve_problem::retrieve))
        .route("/problem/create", post(create_problem::create))
        .layer(middleware::from_fn_with_state(state.clone(), authorizer))
        .route("/user/login", post(login::login))
        .route("/user/signup", post(signup::signup))
        .fallback(async || "Hi! URL not found??")
        .layer(CorsLayer::very_permissive())
        .with_state(state)
}
