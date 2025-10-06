use crate::{error::AppError, routes, utils::app_state::AppState};
use axum::{Router, routing::get};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};

pub async fn app() -> Result<Router, AppError> {
    tracing::info!("Initializing application state");
    let state = AppState::new().await?;

    tracing::info!("Setting up API routes");
    let api_routes = routes::api_routes(&state);

    tracing::info!("Configuring middleware layers");
    let app = Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_check))
        .fallback(fallback_response)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::very_permissive())
                .layer(CookieManagerLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(5))),
        )
        .with_state(state);

    tracing::info!("Application setup complete");
    Ok(app)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn fallback_response() -> AppError {
    AppError::not_found("URL not found")
}
