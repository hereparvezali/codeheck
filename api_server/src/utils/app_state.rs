use super::config::Config;
use crate::error::AppError;
use lapin::{
    Channel, Connection, ConnectionProperties, options::QueueDeclareOptions, types::FieldTable,
};
use rustls::crypto::ring;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::broadcast;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubmissionEvent {
    pub submission_id: i64,
    pub problem_id: i64,
    pub user_id: i64,
    pub status: String,
    pub verdict: Option<String>,
    pub time: Option<i16>,
    pub memory: Option<i16>,
    pub contest_id: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardEvent {
    pub contest_id: i64,
    pub submission_id: i64,
    pub user_id: i64,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub mq: Arc<Channel>,
    pub config: Arc<Config>,
    pub submission_broadcast: Arc<broadcast::Sender<SubmissionEvent>>,
    pub leaderboard_broadcast: Arc<broadcast::Sender<LeaderboardEvent>>,
}

impl AppState {
    pub async fn new() -> Result<AppState, AppError> {
        let config = Config::from_env()
            .map_err(|e| AppError::internal(format!("Failed to load config: {}", e)))?;

        config
            .validate()
            .map_err(|e| AppError::internal(format!("Invalid config: {}", e)))?;

        let mut opt = ConnectOptions::new(config.database.url.clone());
        opt.max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .connect_timeout(Duration::from_secs(config.database.connect_timeout_seconds))
            .idle_timeout(Duration::from_secs(config.database.idle_timeout_seconds))
            .sqlx_logging(true);

        let db = Database::connect(opt)
            .await
            .map_err(|e| AppError::internal(format!("Database connection failed: {}", e)))?;

        ring::default_provider()
            .install_default()
            .map_err(|_| AppError::internal("TLS install failed".to_string()))?;
        let connection = Connection::connect(&config.rabbitmq.url, ConnectionProperties::default())
            .await
            .map_err(|e| AppError::internal(format!("RabbitMQ connection failed: {}", e)))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| AppError::internal(format!("Failed to create RabbitMQ channel: {}", e)))?;

        channel
            .queue_declare(
                config.rabbitmq.outgoing.clone().into(),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| {
                AppError::internal(format!("Failed to declare submissions queue: {}", e))
            })?;

        channel
            .queue_declare(
                config.rabbitmq.incomming.clone().into(),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await
            .map_err(|e| AppError::internal(format!("Failed to declare verdicts queue: {}", e)))?;

        let (submission_tx, _) = broadcast::channel::<SubmissionEvent>(1024);
        let (leaderboard_tx, _) = broadcast::channel::<LeaderboardEvent>(1024);

        Ok(AppState {
            db: Arc::new(db),
            mq: Arc::new(channel),
            config: Arc::new(config),
            submission_broadcast: Arc::new(submission_tx),
            leaderboard_broadcast: Arc::new(leaderboard_tx),
        })
    }
}
