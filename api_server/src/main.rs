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

#[tokio::main]
async fn main() {
    config::load();

    let app = app().await.expect("Failed to build application");

    let bind_addr = "0.0.0.0:8000";

    let tcp_listener = TcpListener::bind(bind_addr)
        .await
        .expect("Failed to bind port");

    axum::serve(tcp_listener, app)
        .with_graceful_shutdown(shutdown::shutdown_signal())
        .await
        .expect("Server failed to start");
}
