use lapin::{
    Channel, Connection, ConnectionProperties, Consumer,
    options::{BasicAckOptions, BasicConsumeOptions, BasicQosOptions, QueueDeclareOptions},
    types::FieldTable,
};
use std::{env, sync::Arc};

use crate::{error::WorkerError, models::SubmissionPublishQueue, pipeline::JudgePipeline};

pub struct QueueConsumer;

impl QueueConsumer {
    pub async fn setup_rabbitmq(
        prefetch_count: u16,
    ) -> Result<(Arc<Channel>, Consumer), WorkerError> {
        dotenvy::dotenv().ok();
        let amqp_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set!");

        let conn = Connection::connect(&amqp_url, ConnectionProperties::default()).await?;
        let channel = conn.create_channel().await?;

        channel
            .queue_declare(
                "outgoing".into(),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_declare(
                "incomming".into(),
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        channel
            .basic_qos(prefetch_count, BasicQosOptions::default())
            .await?;

        let hostname = env::var("HOST").unwrap_or_else(|_| "worker".to_string());
        let consumer = channel
            .basic_consume(
                "outgoing".into(),
                hostname.into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok((Arc::new(channel), consumer))
    }

    pub async fn handle_delivery(
        delivery: lapin::message::Delivery,
        channel: Arc<Channel>,
        core_id: usize,
    ) -> Result<(), WorkerError> {
        let payload: SubmissionPublishQueue = serde_json::from_slice(&delivery.data)?;
        tracing::info!(
            submission_id = payload.submission_id,
            language = %payload.language,
            core_id = core_id,
            testcases = payload.inputs_outputs.len(),
            time_limit = payload.time_limit,
            memory_limit = payload.memory_limit,
            "Received submission job from queue"
        );

        JudgePipeline::process(payload, channel, core_id).await?;
        delivery.ack(BasicAckOptions::default()).await?;

        Ok(())
    }
}
