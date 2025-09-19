use futures_util::StreamExt;
use lapin::{
    Connection, ConnectionProperties,
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, QueueDeclareOptions},
    types::FieldTable,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, process::Stdio, sync::Arc};
use tokio::{
    io::AsyncWriteExt,
    sync::{OwnedSemaphorePermit, Semaphore},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmissionPublishQueue {
    pub submission_id: i64,
    pub problem_id: i64,
    pub language: String,
    pub code: String,
    pub time_limit: i16,
    pub memory_limit: i16,
    pub inputs_outputs: Vec<InputOutput>,
    pub token: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InputOutput {
    pub input: Option<String>,
    pub output: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseFromWorker {
    id: i64,
    status: String,
    verdict: Option<String>,
    time: Option<i16>,
    memory: Option<i16>,
}
impl ResponseFromWorker {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            status: "AC".to_string(),
            verdict: None,
            time: None,
            memory: None,
        }
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let api = Arc::new(env::var("SUBMISSION_API").expect("==> API must set??"));

    let channel = Connection::connect(
        &env::var("RABBITMQ_URL").expect("==> Set RABBITMQ_URL"),
        ConnectionProperties::default(),
    )
    .await
    .expect("==> Rabbitmq not connecting??")
    .create_channel()
    .await
    .expect("Channel cant create??");

    channel
        .queue_declare(
            "submissions",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Queue cant declare??");

    let mut consumer = channel
        .basic_consume(
            "submissions",
            &env::var("HOSTNAME").expect("==> HOSTNAME not set??"),
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Setup consumer failed??");

    let semaphore = Arc::new(Semaphore::new(num_cpus::get()));
    while let Some(Ok(delivery)) = consumer.next().await {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        tokio::spawn(handle_delivery(delivery, api.clone(), permit));
    }
}

fn ext(language: &str) -> &str {
    match language {
        "cpp" | "c++" => "cpp",
        "python" => "py",
        "java" => "java",
        "rust" | "Rust" => "rs",
        _ => "txt",
    }
}
fn image(language: &str) -> &str {
    match language {
        "cpp" | "c++" | "C++" => "gcc:trixie",
        "python" | "py" => "python:slim",
        "java" => "openjdk:slim",
        _ => "rust:slim",
    }
}
pub async fn handle_delivery(delivery: Delivery, api: Arc<String>, _permit: OwnedSemaphorePermit) {
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
        let output = run_in_docker(
            image,
            &code_path,
            &case.input,
            payload.time_limit,
            payload.memory_limit,
            &payload.language,
        )
        .await;

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

pub async fn run_in_docker(
    image: &str,
    code_path: &str,
    input: &Option<String>,
    time_limit_ms: i16,
    memory_limit: i16,
    language: &str,
) -> String {
    // Determine compile & run commands
    let (compile_cmd, run_cmd) = match language.to_lowercase().as_str() {
        "cpp" | "c++" => (
            Some("g++ /code/code.cpp -o /code/a.out".to_string()),
            "/code/a.out".to_string(),
        ),
        "rust" => (
            Some("rustc /code/code.rs -o /code/a.out".to_string()),
            "/code/a.out".to_string(),
        ),
        "java" => (
            Some("javac /code/code.java".to_string()),
            "java -cp /code code".to_string(),
        ),
        "python" | "py" => (None, "python3 /code/code.py".to_string()),
        _ => (None, "cat /code/code".to_string()),
    };

    // Convert time limit to seconds as float with precision
    let time_sec = time_limit_ms as f64 / 1000.0;

    // Combine compile + run in single bash command
    let mut full_cmd = String::new();
    if let Some(ref compile) = compile_cmd {
        // Compilation first, if fails, container exits with non-zero
        full_cmd.push_str(&format!("{} && ", compile));
    }
    // Runtime with timeout using seconds as floating point
    full_cmd.push_str(&format!("timeout -s 9 {}s {}", time_sec, run_cmd));

    // Spawn single Docker container
    let mut child = tokio::process::Command::new("docker")
        .args([
            "run",
            "--rm",
            "-i", // interactive for stdin
            "-v",
            &format!("{}:/code/code.{}", code_path, ext(language)),
            "--cpus=1",
            &format!("--memory={}m", memory_limit),
            "--network=none",
            image,
            "bash",
            "-c",
            &full_cmd,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn docker");

    // Write input if provided
    if let Some(mut stdin) = child.stdin.take() {
        if let Some(data) = input {
            stdin.write_all(data.as_bytes()).await.unwrap();
        }
    }

    let output = child.wait_with_output().await.unwrap();

    // Handle exit codes
    if output.status.code() == Some(124) {
        return "TLE".to_string(); // Time Limit Exceeded
    } else if !output.status.success() {
        // If compiled language, failed compilation or runtime
        if compile_cmd.is_some() {
            return "CE/RE".to_string();
        } else {
            return "RE".to_string();
        }
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}
