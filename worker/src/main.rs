mod consumer;
mod error;
mod languages;
mod models;
mod pipeline;
mod sandbox;

use futures_util::StreamExt as _;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tracing_subscriber::{EnvFilter, fmt};

use crate::{consumer::QueueConsumer, error::WorkerError, sandbox::IsolateSandbox};

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

    tracing::info!("Starting...");

    // 1. Verify that the isolate executable is present on the host
    IsolateSandbox::verify_environment().await?;

    let max_cpu_id = num_cpus::get() - 1;

    // 2. Clean up any leftover isolate boxes from prior runs
    for box_id in 0..max_cpu_id {
        let _ = IsolateSandbox::cleanup_box(box_id).await;
    }
    tracing::info!(slots = max_cpu_id, "Initialized isolate sandbox slots");

    // 3. Connect to RabbitMQ
    let (channel, mut consumer) = QueueConsumer::setup_rabbitmq(max_cpu_id as u16).await?;

    let semaphore = Arc::new(Semaphore::new(max_cpu_id));
    let core_counter = Arc::new(Mutex::new(0));

    tracing::info!(
        slots = max_cpu_id,
        queue = "outgoing",
        "CodeHeck Judge Worker ready. Listening for submissions on 'outgoing' queue..."
    );

    while let Some(Ok(delivery)) = consumer.next().await {
        let ch = channel.clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        let core_id = {
            let mut counter = core_counter.lock().await;
            let id = *counter;
            *counter = (*counter + 1) % max_cpu_id;
            id + 1
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
