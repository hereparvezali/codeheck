use std::time::Duration;

use crate::{
    contest::{
        add_contest_problem, create_contest, retrieve_contest, retrieve_contest_problems,
        retrieve_contest_submissions,
    },
    problem::{create_problem, create_testcase, retrieve_problem},
    submission::{create_submission, retrieve_submission},
    user::{
        login, retrieve_user_created_problems, retrieve_user_info, retrieve_user_solved_problems,
        retrieve_user_submissions, signup,
    },
    utils::{app_state::AppState, middlewares::authorizer},
};
use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

pub async fn app() -> Router {
    let state = AppState::new().await;
    Router::new()
        .route(
            "/submission/retrieve/{id}",
            get(retrieve_submission::retrieve),
        )
        .route("/submission/create", post(create_submission::create))
        .route(
            "/contest/retrieve_submissions",
            get(retrieve_contest_submissions::retrieve),
        )
        .route("/contest/add_problem", get(add_contest_problem::add))
        .route(
            "/contest/retrieve_problems",
            get(retrieve_contest_problems::retrieve),
        )
        .route("/contest/retrieve", get(retrieve_contest::retrieve))
        .route("/contest/create", post(create_contest::create))
        .route("/problem/create_testcase", post(create_testcase::create))
        .route("/problem/retrieve", get(retrieve_problem::retrieve))
        .route("/problem/create", post(create_problem::create))
        .route(
            "/user/retrieve_submissions",
            get(retrieve_user_submissions::retrieve),
        )
        .route(
            "/user/retrieve_solved",
            get(retrieve_user_solved_problems::retrieve),
        )
        .route(
            "/user/retrieve_created",
            get(retrieve_user_created_problems::retrieve),
        )
        .route("/user/retrieve", get(retrieve_user_info::retrieve))
        .layer(middleware::from_fn_with_state(state.clone(), authorizer))
        .route("/user/login", post(login::login))
        .route("/user/signup", post(signup::signup))
        .fallback(fallback_response)
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .layer(CorsLayer::permissive()),
        )
        .with_state(state)
}

async fn fallback_response() -> &'static str {
    "Hi! URL not found??"
}
