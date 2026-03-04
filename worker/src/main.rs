mod docker;
mod language;
mod models;
mod queue;

use crate::queue::handle;
use futures_util::StreamExt as _;
use std::sync::Arc;
use tokio::{
    fs,
    sync::{Mutex, Semaphore},
};

#[tokio::main]
async fn main() {
    // Build Docker images
    if let Err(e) = docker::build_images().await {
        println!("Failed to build Docker images: {:?}", e);
    }

    // Setup RabbitMQ
    let (api, _, mut consumer) = match queue::setup_rabbitmq().await {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to setup RabbitMQ: {:?}", e);
            return;
        }
    };

    // Ensure /tmp/codebox exists
    if !fs::try_exists("/tmp/codebox").await.unwrap() {
        fs::create_dir("/tmp/codebox").await.unwrap();
    }

    // Semaphore for spawning not more than core_count tast and core count for dedicating containers per core
    let cpus = num_cpus::get() - 1; // skipped 1 core for relaxation
    let semaphore = Arc::new(Semaphore::new(cpus));
    let core_counter = Arc::new(Mutex::new(0)); // Core pool

    // Process messages from the queue
    while let Some(Ok(delivery)) = consumer.next().await {
        let api = api.clone();
        let semaphore = semaphore.clone();

        // extra scopping for unlock the mutex
        let core_id = {
            let mut counter = core_counter.lock().await;
            let id = *counter;
            *counter = (*counter + 1) % cpus; // round-robin assignment
            id
        };

        // spawnning at most number of cores task
        tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            if let Err(e) = handle(delivery, api, core_id).await {
                println!("Error handling delivery: {:?}", e);
            }
        });
    }
}
