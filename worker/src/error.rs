use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum WorkerError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("RabbitMQ error: {0}")]
    RabbitMq(#[from] lapin::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Sandbox error: {0}")]
    Sandbox(String),

    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    #[error("Execution error: {0}")]
    Execution(String),
}
