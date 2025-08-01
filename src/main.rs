use crate::{app::app, utils::config};
use tokio::net::TcpListener;

pub mod app;
pub mod dto;
pub mod entity;
pub mod user;
pub mod utils;
pub mod problem;
pub mod contest;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // tracing_subscriber::fmt::init();
    config::load();

    let tcp_listener = TcpListener::bind("localhost:8000").await?;

    axum::serve(tcp_listener, app().await).await?;

    Ok(())
}
