mod docker;
mod language;
mod models;
mod queue;

use std::sync::Arc;

use futures_util::StreamExt as _;
use tokio::sync::{Mutex, Semaphore};

use crate::queue::handle_delivery;

#[tokio::main]
async fn main() {
    // Build Docker images
    if let Err(e) = docker::build_images().await {
        eprintln!("Failed to build Docker images: {}", e);
        return;
    }

    // Setup RabbitMQ
    let (api, _, mut consumer) = match queue::setup_rabbitmq().await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to setup RabbitMQ: {}", e);
            return;
        }
    };

    // Semaphore for spawning not more than core_count tast and core count for dedicating containers per core
    let cpus = num_cpus::get();
    let semaphore = Arc::new(Semaphore::new(cpus));
    let core_counter = Arc::new(Mutex::new(0));

    // Process messages from the queue
    while let Some(Ok(delivery)) = consumer.next().await {
        let api = api.clone();
        let semaphore = semaphore.clone();
        let core_counter = core_counter.clone();

        tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            let mut counter = core_counter.lock().await;
            *counter = (*counter + 1) % cpus;

            if let Err(e) = handle_delivery(delivery, api, *counter).await {
                eprintln!("Error handling delivery: {}", e);
            }
        });
    }
}
