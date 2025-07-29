use std::{env, sync::Arc};

use axum::{Router, routing::get};
use dotenvy::dotenv;
use entity::prelude::*;
use sea_orm::{Database, DatabaseConnection};
use tokio::{net::TcpListener, sync::Mutex};

pub mod entity;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Arc<Mutex<DatabaseConnection>>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;

    let app: Router = Router::new()
        .route("/", get(async || "Hellow there!!".to_string()))
        .fallback(async || "Maybe Invalid URL🤔".to_string())
        .with_state(AppState {
            db: Arc::new(Mutex::new(db)),
        });

    let tcp_listener = TcpListener::bind("localhost:8000").await?;
    axum::serve(tcp_listener, app).await?;
    Ok(())
}
