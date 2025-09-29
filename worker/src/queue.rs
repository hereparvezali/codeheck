use crate::docker::run;
use crate::language::ext;
use crate::models::{ResponseFromWorker, SubmissionPublishQueue};
use anyhow::Result;
use dotenvy::dotenv;
use lapin::{
    Channel, Connection, ConnectionProperties, Consumer,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
};
use std::sync::Arc;

/// Initializes the RabbitMQ connection and declares the queue.
pub async fn setup_rabbitmq() -> Result<(Arc<String>, Channel, Consumer)> {
    dotenv().ok();

    let api = Arc::new(std::env::var("SUBMISSION_API")?);
    let rabbitmq_url = std::env::var("RABBITMQ_URL")?;
    let hostname = std::env::var("HOSTNAME")?;

    let connection = Connection::connect(&rabbitmq_url, ConnectionProperties::default()).await?;
    let channel = connection.create_channel().await?;
    channel
        .queue_declare(
            "submissions",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    let consumer = channel
        .basic_consume(
            "submissions",
            &hostname,
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    Ok((api, channel, consumer))
}

/// Handles incoming messages from the RabbitMQ queue.
pub async fn handle_delivery(delivery: Delivery, api: Arc<String>, core_id: usize) -> Result<()> {
    let payload: SubmissionPublishQueue = serde_json::from_slice(&delivery.data)?;
    let code_path = format!(
        "/tmp/submission_{}.{}",
        payload.submission_id,
        ext(&payload.language)
    );
    tokio::fs::write(&code_path, &payload.code).await?;

    let mut response = ResponseFromWorker::new(payload.submission_id);
    println!("{}", core_id);
    'case_loop: for (case_num, case) in payload.inputs_outputs.iter().enumerate() {
        let output = run(&code_path, &case.input, &payload, core_id)
            .await
            .unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        for line in stderr.lines() {
            if line.starts_with("Command terminated by signal 9") {
                response.status = "TLE".to_owned();
                response.verdict = Some(format!("Time limit exited in case {}", case_num));
            }

            if line.contains("Maximum resident set size") {
                if let Some(mem_kb) = line
                    .split(':')
                    .last()
                    .and_then(|x| x.trim().parse::<i32>().ok())
                {
                    let mem_mb = (mem_kb / 1000) as i16;
                    response.memory = Some(response.memory.map_or(mem_mb, |curr| curr.max(mem_mb)));
                }
            }

            if line.contains("Elapsed (wall clock) time") {
                if let Some(ms) = line
                    .split_whitespace()
                    .last()
                    .and_then(|x| {
                        x.trim_end_matches('s')
                            .trim_start_matches("0:")
                            .parse::<f32>()
                            .ok()
                    })
                    .map(|secs| (secs * 1000.0) as i16)
                {
                    response.time = Some(response.time.map_or(ms, |curr| curr.max(ms)));
                }
            }
        }

        if response.time.is_none() && response.memory.is_none() {
            response.status = "CE/RE".to_string();
            response.verdict = Some(stderr);
            break 'case_loop;
        }

        if response.status == "AC"
            && case
                .output
                .as_ref()
                .map_or("".to_string(), |x| x.trim().to_string())
                != String::from_utf8(output.stdout)
                    .unwrap_or_default()
                    .trim_end_matches('\n')
                    .trim()
                    .to_string()
        {
            response.status = "WA".to_string();
            response.verdict = Some(format!("Wrong answer on testcase: {}", case_num));
            break 'case_loop;
        }
    }

    reqwest::Client::new()
        .put(api.to_string())
        .json(&response)
        .bearer_auth(payload.token)
        .send()
        .await?;

    delivery.ack(BasicAckOptions::default()).await?;
    tokio::fs::remove_file(code_path).await?;

    Ok(())
}
