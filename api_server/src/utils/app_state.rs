use super::config::Config;
use crate::error::AppError;
use lapin::{
    Channel, Connection, ConnectionProperties, options::QueueDeclareOptions, types::FieldTable,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{sync::Arc, time::Duration};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
    pub mq: Arc<Channel>,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new() -> Result<AppState, AppError> {
        let config = Config::from_env()
            .map_err(|e| AppError::internal(format!("Failed to load config: {}", e)))?;

        config
            .validate()
            .map_err(|e| AppError::internal(format!("Invalid config: {}", e)))?;

        tracing::info!("Configuration loaded and validated");

        let mut opt = ConnectOptions::new(config.database.url.clone());
        opt.max_connections(config.database.max_connections)
            .min_connections(config.database.min_connections)
            .connect_timeout(Duration::from_secs(config.database.connect_timeout_seconds))
            .idle_timeout(Duration::from_secs(config.database.idle_timeout_seconds))
            .sqlx_logging(true);

        tracing::info!("Connecting to database...");
        let db = Database::connect(opt)
            .await
            .map_err(|e| AppError::internal(format!("Database connection failed: {}", e)))?;

        tracing::info!("Database connected successfully");

        tracing::info!("Connecting to RabbitMQ...");
        let connection = Connection::connect(&config.rabbitmq.url, ConnectionProperties::default())
            .await
            .map_err(|e| AppError::internal(format!("RabbitMQ connection failed: {}", e)))?;

        let channel = connection
            .create_channel()
            .await
            .map_err(|e| AppError::internal(format!("Failed to create RabbitMQ channel: {}", e)))?;

        channel
            .queue_declare(
                &config.rabbitmq.queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .map_err(|e| AppError::internal(format!("Failed to declare queue: {}", e)))?;

        tracing::info!("RabbitMQ connected successfully");

        Ok(AppState {
            db: Arc::new(db),
            mq: Arc::new(channel),
            config: Arc::new(config),
        })
    }
}
