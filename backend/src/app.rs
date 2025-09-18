use crate::{
    contest::{
        add_contest_problem, create_contest, create_register, delete_register, retrieve_contest,
        retrieve_contest_problems, retrieve_contest_submissions, retrieve_contest_user_submissions,
        retrieve_contests,
    },
    problem::{create_problem, create_testcases, retrieve_problem, retrieve_problems},
    submission::{create_submission, retrieve_submission},
    user::{
        refresh_access_token, retrieve_user, retrieve_user_contests, retrieve_user_info,
        retrieve_user_problems, retrieve_user_solved_problems, retrieve_user_submissions, signin,
        signout, signup,
    },
    utils::{
        app_state::AppState,
        middlewares::{authorizer, giving_delay},
    },
};
use axum::{
    Router, middleware,
    routing::{delete, get, post},
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
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
            "/contest/delete_register/{id}",
            delete(delete_register::delete),
        )
        .route(
            "/contest/create_register/{contest_id}",
            post(create_register::create),
        )
        .route(
            "/contest/retrieve_contests",
            get(retrieve_contests::retrieve),
        )
        .route(
            "/contest/retrieve_user_submissions",
            get(retrieve_contest_user_submissions::retrieve),
        )
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
        .route("/problem/create_testcases", post(create_testcases::create))
        .route(
            "/problem/retrieve_problems",
            get(retrieve_problems::retrieve),
        )
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
            "/user/retrieve_contests",
            get(retrieve_user_contests::retrieve),
        )
        .route(
            "/user/retrieve_problems",
            get(retrieve_user_problems::retrieve),
        )
        .route("/user/retrieve", get(retrieve_user::retrieve))
        .route("/user/retrieve_user", get(retrieve_user_info::retrieve))
        .layer(middleware::from_fn_with_state(state.clone(), authorizer))
        .route("/user/signout", get(signout::signout))
        .route("/user/refresh", get(refresh_access_token::refresh))
        .route("/user/signin", post(signin::signin))
        .route("/user/signup", post(signup::signup))
        .fallback(fallback_response)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive())
                .layer(CookieManagerLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        )
        .layer(middleware::from_fn(giving_delay))
        .with_state(state)
}

async fn fallback_response() -> &'static str {
    "Hi! URL not found??"
}
