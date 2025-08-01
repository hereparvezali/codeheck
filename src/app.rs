use axum::{Router, middleware, routing::post};

use crate::{
    problem::create_problem,
    user::{login, signup},
    utils::{app_state::AppState, middlewares::authorizer},
};

pub async fn app() -> Router {
    let state = AppState::new().await;
    Router::new()
        .route("/problem/create", post(create_problem::create_problem))
        .layer(middleware::from_fn_with_state(state.clone(), authorizer))
        .route("/user/login", post(login::login))
        .route("/user/signup", post(signup::signup))
        .with_state(state)
}
