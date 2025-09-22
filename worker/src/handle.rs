use crate::{
    languages::{ext, image},
    run::run_in_docker,
    types::{ResponseFromWorker, SubmissionPublishQueue},
};
use lapin::{message::Delivery, options::BasicAckOptions};
use reqwest::Client;
use std::{collections::VecDeque, sync::Arc};
use tokio::sync::{Mutex, OwnedSemaphorePermit};

pub async fn handle_delivery(
    delivery: Delivery,
    api: Arc<String>,
    _permit: OwnedSemaphorePermit,
    core_pool: Arc<Mutex<VecDeque<usize>>>,
) {
    let payload: SubmissionPublishQueue = serde_json::from_slice(&delivery.data).unwrap();

    let code_path = format!(
        "/tmp/submission_{}.{}",
        payload.submission_id,
        ext(&payload.language)
    );
    tokio::fs::write(&code_path, &payload.code).await.unwrap();
    let image = image(&payload.language);

    let mut response = ResponseFromWorker::new(payload.submission_id);

    for (case_num, case) in payload.inputs_outputs.iter().enumerate() {
        // 🔹 acquire a core
        let core_id = {
            let mut pool = core_pool.lock().await;
            pool.pop_front().expect("No cores available!")
        };

        let output = run_in_docker(
            image,
            &code_path,
            &case.input,
            payload.time_limit,
            payload.memory_limit,
            &payload.language,
            core_id,
        )
        .await;

        // 🔹 release the core
        {
            let mut pool = core_pool.lock().await;
            pool.push_back(core_id);
        }

        if output.trim_end_matches("\n").trim()
            != case
                .output
                .as_deref()
                .unwrap_or_default()
                .trim_end_matches("\n")
                .trim()
        {
            response.status = "WA".to_string();
            response.verdict = Some(format!("WA in case number: {}", case_num));
            break;
        }
    }

    Client::new()
        .put(api.to_string())
        .json(&response)
        .bearer_auth(payload.token)
        .send()
        .await
        .unwrap();

    delivery.ack(BasicAckOptions::default()).await.unwrap();
    tokio::fs::remove_file(code_path).await.unwrap();
}
