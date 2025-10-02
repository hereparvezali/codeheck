use crate::{
    routes,
    utils::{app_state::AppState, logger},
};
use axum::{Router, middleware, routing::get};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

pub async fn app() -> Router {
    let state = AppState::new().await;
    
    // Get all API routes (protected + public)
    let api_routes = routes::api_routes(&state);
    
    // Main router with middleware layers
    Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_check))
        .fallback(fallback_response)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::very_permissive())
                .layer(CookieManagerLayer::new())
                .layer(TimeoutLayer::new(Duration::from_secs(5)))
                .layer(middleware::from_fn(logger::logger)),
        )
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}

async fn fallback_response() -> &'static str {
    "Hi! URL not found??"
}
