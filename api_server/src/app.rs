use crate::{error::AppError, routes, utils::{app_state::AppState, logger}};
use axum::{Router, http::StatusCode, middleware, routing::get};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

pub async fn app() -> Result<Router, AppError> {
    let state = AppState::new().await?;


    crate::submission::consumer::VerdictsConsumer::start(state.clone());

    let api_routes = routes::api_routes(&state);

    let app = Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_check))
        .fallback(fallback_response)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive())
                .layer(CookieManagerLayer::new())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::SERVICE_UNAVAILABLE,
                    Duration::from_secs(state.config.server.timeout_seconds),
                ))
                .layer(middleware::from_fn(logger::logger)),
        )
        .with_state(state);

    Ok(app)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn fallback_response() -> AppError {
    AppError::not_found("URL not found")
}
