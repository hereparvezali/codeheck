use crate::{app::app, utils::config};
use dto::MyErr;
use tokio::net::TcpListener;

pub mod app;
pub mod contest;
pub mod dto;
#[path = "../entity/mod.rs"]
pub mod entity;
pub mod problem;
pub mod submission;
pub mod user;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), MyErr> {
    config::load();

    let tcp_listener = TcpListener::bind("localhost:8000")
        .await
        .map_err(|e| MyErr::Unknown(e.to_string()))?;

    axum::serve(tcp_listener, app().await)
        .await
        .map_err(|e| MyErr::Unknown(e.to_string()))?;

    Ok(())
}
