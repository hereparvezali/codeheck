#![allow(unused)]

use std::{env, sync::Arc, time::Duration};

use axum::{Router, routing::get};
use dotenvy::dotenv;
use tokio::{net::TcpListener, sync::Mutex};

#[derive(Debug, Clone)]
pub struct AppState {
    // pub db: Arc<Mutex<Pool<MySql>>>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let app: Router = Router::new()
        .route("/", get(async || "Hellow there!!".to_string()))
        .fallback(async || "Maybe Invalid URL🤔".to_string())
        .with_state(AppState {});

    let tcp_listener = TcpListener::bind("localhost:8000").await?;
    axum::serve(tcp_listener, app).await?;

    Ok(())
}
