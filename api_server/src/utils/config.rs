use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub rabbitmq: RabbitMqConfig,
    pub jwt: JwtConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_seconds: u64,
    pub idle_timeout_seconds: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RabbitMqConfig {
    pub url: String,
    pub queue_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry_minutes: usize,
    pub refresh_token_expiry_minutes: usize,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8000".to_string())
                    .parse()
                    .map_err(|_| "Invalid SERVER_PORT")?,
                timeout_seconds: env::var("SERVER_TIMEOUT")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .map_err(|_| "Invalid SERVER_TIMEOUT")?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?,
                max_connections: env::var("DB_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .map_err(|_| "Invalid DB_MAX_CONNECTIONS")?,
                min_connections: env::var("DB_MIN_CONNECTIONS")
                    .unwrap_or_else(|_| "2".to_string())
                    .parse()
                    .map_err(|_| "Invalid DB_MIN_CONNECTIONS")?,
                connect_timeout_seconds: env::var("DB_CONNECT_TIMEOUT")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .map_err(|_| "Invalid DB_CONNECT_TIMEOUT")?,
                idle_timeout_seconds: env::var("DB_IDLE_TIMEOUT")
                    .unwrap_or_else(|_| "600".to_string())
                    .parse()
                    .map_err(|_| "Invalid DB_IDLE_TIMEOUT")?,
            },
            rabbitmq: RabbitMqConfig {
                url: env::var("RABBITMQ_URL").map_err(|_| "RABBITMQ_URL must be set")?,
                queue_name: env::var("RABBITMQ_QUEUE")
                    .unwrap_or_else(|_| "submissions".to_string()),
            },
            jwt: JwtConfig {
                secret: env::var("SECRET").map_err(|_| "SECRET must be set")?,
                access_token_expiry_minutes: env::var("JWT_ACCESS_EXPIRY")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()
                    .map_err(|_| "Invalid JWT_ACCESS_EXPIRY")?,
                refresh_token_expiry_minutes: env::var("JWT_REFRESH_EXPIRY")
                    .unwrap_or_else(|_| "10080".to_string())
                    .parse()
                    .map_err(|_| "Invalid JWT_REFRESH_EXPIRY")?,
            },
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.server.port == 0 {
            return Err("Server port cannot be 0".to_string());
        }

        if self.database.max_connections < self.database.min_connections {
            return Err("Max connections must be >= min connections".to_string());
        }

        if self.jwt.secret.len() < 8 {
            return Err("JWT secret must be at least 16 characters".to_string());
        }

        Ok(())
    }
}

pub fn load() {
    dotenvy::dotenv().ok();
    tracing::info!("Environment variables loaded");
}
