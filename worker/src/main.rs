mod consumer;
mod error;
mod languages;
mod models;
mod pipeline;
mod sandbox;

use futures_util::StreamExt as _;
use std::sync::Arc;
use tokio::{
    fs,
    sync::{Mutex, Semaphore},
};
use tracing_subscriber::{EnvFilter, fmt};

use crate::{consumer::QueueConsumer, error::WorkerError, sandbox::DockerSandbox};

#[tokio::main]
async fn main() -> Result<(), WorkerError> {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("worker=info,info")),
        )
        .with_target(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    tracing::info!("CodeHeck Judge Engine Worker Starting");

    if let Err(e) = DockerSandbox::build_compiler_images().await {
        tracing::warn!("Docker images build report: {:?}", e);
    }

    if !fs::try_exists("/tmp/codebox").await.unwrap_or(false) {
        fs::create_dir_all("/tmp/codebox").await?;
    }

    let cpus = (num_cpus::get().saturating_sub(1)).max(1);
    let (channel, mut consumer) = QueueConsumer::setup_rabbitmq(cpus as u16).await?;

    let semaphore = Arc::new(Semaphore::new(cpus));
    let core_counter = Arc::new(Mutex::new(0));

    tracing::info!(
        slots = cpus,
        queue = "outgoing",
        "CodeHeck Judge Worker ready. Listening for submissions on 'outgoing' queue..."
    );

    while let Some(Ok(delivery)) = consumer.next().await {
        let ch = channel.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let core_id = {
            let mut counter = core_counter.lock().await;
            let id = *counter;
            *counter = (*counter + 1) % cpus;
            id
        };

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = QueueConsumer::handle_delivery(delivery, ch, core_id).await {
                tracing::error!(core_id, error = ?e, "Delivery processing failed");
            }
        });
    }

    Ok(())
}
