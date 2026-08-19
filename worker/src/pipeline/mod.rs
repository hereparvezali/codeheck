use std::sync::Arc;
use tokio::fs;

use crate::{
    error::WorkerError,
    languages::LanguageRegistry,
    models::{ResponseFromWorker, SubmissionPublishQueue},
    sandbox::DockerSandbox,
};

pub struct JudgePipeline;

impl JudgePipeline {
    pub fn normalize_output(s: &str) -> String {
        s.lines()
            .map(|l| l.trim_end())
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string()
    }


    pub async fn process(
        payload: SubmissionPublishQueue,
        channel: Arc<lapin::Channel>,
        core_id: usize,
    ) -> Result<(), WorkerError> {
        let strategy = match LanguageRegistry::get(&payload.language) {
            Some(s) => s,
            None => {
                let mut resp = ResponseFromWorker::new(payload.submission_id);
                resp.status = "CE".to_string();
                resp.verdict = Some(format!("Unsupported language: {}", payload.language));
                Self::send_verdict(&channel, &resp).await?;
                return Ok(());
            }
        };

        let job_dir = format!("/tmp/codebox/{}", payload.submission_id);
        let code_path = format!("{}/Main.{}", job_dir, strategy.file_extension());


        if !fs::try_exists(&job_dir).await.unwrap_or(false) {
            fs::create_dir_all(&job_dir).await?;
        }
        fs::write(&code_path, &payload.code).await?;

        let mut response = ResponseFromWorker::new(payload.submission_id);


        if let Some(compile_output) =
            DockerSandbox::compile(&strategy, payload.submission_id, core_id).await?
        {
            if !compile_output.status.success() {
                let stderr = String::from_utf8_lossy(&compile_output.stderr);
                response.status = "CE".to_string();
                response.verdict = Some(stderr.chars().take(2000).collect());

                Self::send_verdict(&channel, &response).await?;
                let _ = fs::remove_dir_all(&job_dir).await;
                return Ok(());
            }
        }


        let mut max_time: i16 = 0;
        let mut max_mem: i16 = 0;

        for (case_num, case) in payload.inputs_outputs.iter().enumerate() {
            let output = DockerSandbox::execute_case(
                &strategy,
                payload.submission_id,
                &case.input,
                payload.time_limit,
                payload.memory_limit,
                core_id,
            )
            .await?;

            let stderr = String::from_utf8_lossy(&output.stderr);
            let exit_code = output.status.code().unwrap_or(0);
            let mut is_tle = exit_code == 124 || exit_code == 137;
            let mut case_time: Option<i16> = None;
            let mut case_memory: Option<i16> = None;

            for line in stderr.lines() {
                if line.contains("Command terminated by signal 9")
                    || line.contains("Command terminated by signal 15")
                    || line.contains("Command timed out")
                {
                    is_tle = true;
                }
                if line.starts_with("RESULT_TIME_MS:") {
                    if let Ok(t) = line
                        .trim_start_matches("RESULT_TIME_MS:")
                        .trim()
                        .parse::<i16>()
                    {
                        case_time = Some(t);
                    }
                }
                if line.starts_with("RESULT_MEM_KB:") {
                    if let Ok(m) = line
                        .trim_start_matches("RESULT_MEM_KB:")
                        .trim()
                        .parse::<i16>()
                    {
                        case_memory = Some(m);
                    }
                }
            }

            let t = case_time.unwrap_or(0);
            let m = case_memory.unwrap_or(0);
            max_time = max_time.max(t);
            max_mem = max_mem.max(m);

            if is_tle || t > payload.time_limit {
                response.status = "TLE".to_string();
                response.verdict =
                    Some(format!("Time limit exceeded on testcase {}", case_num + 1));
                break;
            }

            if !output.status.success() {
                response.status = "RE".to_string();
                response.verdict = Some(format!(
                    "Runtime error on testcase {}: {}",
                    case_num + 1,
                    stderr.chars().take(200).collect::<String>()
                ));
                break;
            }

            let expected = Self::normalize_output(case.output.as_deref().unwrap_or(""));
            let actual = Self::normalize_output(&String::from_utf8_lossy(&output.stdout));

            if expected != actual {
                response.status = "WA".to_string();
                response.verdict = Some(format!("Wrong answer on testcase {}", case_num + 1));
                break;
            }
        }

        if response.status == "AC" {
            response.verdict = Some("Accepted".to_string());
        }
        response.time = Some(max_time);
        response.memory = Some(max_mem);


        Self::send_verdict(&channel, &response).await?;


        let _ = fs::remove_dir_all(&job_dir).await;

        Ok(())
    }

    async fn send_verdict(
        channel: &lapin::Channel,
        response: &ResponseFromWorker,
    ) -> Result<(), WorkerError> {
        tracing::info!(
            submission_id = response.id,
            status = %response.status,
            verdict = ?response.verdict,
            time_ms = ?response.time,
            memory_kb = ?response.memory,
            "Evaluation complete. Dispatching verdict to 'incomming' queue"
        );

        let payload_bytes = serde_json::to_vec(response)?;

        channel
            .basic_publish(
                "".into(),
                "incomming".into(),
                lapin::options::BasicPublishOptions::default(),
                &payload_bytes,
                lapin::BasicProperties::default().with_delivery_mode(2),
            )
            .await?;

        tracing::debug!(submission_id = response.id, "Verdict published to RabbitMQ 'incomming' queue");
        Ok(())
    }
}
