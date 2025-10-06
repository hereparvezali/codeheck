pub mod app;
pub mod contest;
#[path = "../entity/mod.rs"]
pub mod entity;
pub mod error;
pub mod problem;
pub mod routes;
pub mod submission;
pub mod user;
pub mod utils;

use crate::{
    app::app,
    utils::{config, shutdown},
};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "codeheck=debug,tower_http=debug,sea_orm=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting CodeHeck application");

    config::load();

    tracing::info!("Building application router");
    let app = app().await.expect("Failed to build application");

    let bind_addr = "0.0.0.0:8000";
    tracing::info!("Binding to {}", bind_addr);

    let tcp_listener = TcpListener::bind(bind_addr)
        .await
        .expect("Failed to bind port");

    tracing::info!("Server listening on {}", bind_addr);
    tracing::info!("Health check available at http://{}/health", bind_addr);
    tracing::info!("API available at http://{}/api", bind_addr);

    // Serve with graceful shutdown
    axum::serve(tcp_listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .expect("Server failed to start");

    tracing::info!("Server stopped gracefully");
}
