mod docker;
mod language;
mod models;
mod queue;

use futures_util::StreamExt as _;

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

    // Process messages from the queue
    while let Some(Ok(delivery)) = consumer.next().await {
        let api = api.clone();

        tokio::spawn(async move {
            if let Err(e) = queue::handle_delivery(delivery, api).await {
                eprintln!("Error handling delivery: {}", e);
            }
        });
    }
}
