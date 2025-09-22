pub mod app;
pub mod contest;
pub mod dto;
#[path = "../entity/mod.rs"]
pub mod entity;
pub mod problem;
pub mod submission;
pub mod user;
pub mod utils;

use crate::{app::app, utils::config};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    config::load();

    let tcp_listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Port can't bind. Maybe the port is already binded!!");

    axum::serve(tcp_listener, app().await)
        .await
        .expect("Axum cant serve the application!!");
}
