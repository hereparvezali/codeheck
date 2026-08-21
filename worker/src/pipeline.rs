use std::sync::Arc;

use crate::{
    error::WorkerError,
    languages::LanguageRegistry,
    models::{ResponseFromWorker, SubmissionPublishQueue},
    sandbox::IsolateSandbox,
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

        // 1. Initialize Isolate sandbox box for this core slot
        let box_dir = match IsolateSandbox::init_box(core_id).await {
            Ok(dir) => dir,
            Err(e) => {
                tracing::error!(core_id, error = ?e, "Failed to init isolate box");
                let mut resp = ResponseFromWorker::new(payload.submission_id);
                resp.status = "RE".to_string();
                resp.verdict = Some(format!("Sandbox initialization error: {}", e));
                Self::send_verdict(&channel, &resp).await?;
                return Ok(());
            }
        };

        let src_filename = format!("Main.{}", strategy.file_extension());
        let src_path = box_dir.join(&src_filename);
        let out_path = if strategy.name() == "Java" {
            box_dir.clone()
        } else {
            box_dir.join("Main")
        };

        if let Err(e) = tokio::fs::write(&src_path, &payload.code).await {
            let _ = IsolateSandbox::cleanup_box(core_id).await;
            return Err(WorkerError::Io(e));
        }

        let mut response = ResponseFromWorker::new(payload.submission_id);

        // 2. Compilation Stage (if applicable)
        match IsolateSandbox::compile(strategy.as_ref(), &src_path, &out_path).await {
            Ok(Some(compile_output)) => {
                if !compile_output.status.success() {
                    let stderr = String::from_utf8_lossy(&compile_output.stderr);
                    response.status = "CE".to_string();
                    response.verdict = Some(stderr.chars().take(2000).collect());

                    Self::send_verdict(&channel, &response).await?;
                    let _ = IsolateSandbox::cleanup_box(core_id).await;
                    return Ok(());
                }
            }
            Err(e) => {
                response.status = "CE".to_string();
                response.verdict = Some(format!("Compilation failure: {}", e));

                Self::send_verdict(&channel, &response).await?;
                let _ = IsolateSandbox::cleanup_box(core_id).await;
                return Ok(());
            }
            Ok(None) => {}
        }

        // 3. Execution & Evaluation Stage per Testcase
        let mut max_time: i16 = 0;
        let mut max_mem: i16 = 0;

        for (case_num, case) in payload.inputs_outputs.iter().enumerate() {
            let result = match IsolateSandbox::execute_case(
                strategy.as_ref(),
                core_id,
                &box_dir,
                &case.input,
                payload.time_limit,
                payload.memory_limit,
            )
            .await
            {
                Ok(res) => res,
                Err(e) => {
                    response.status = "RE".to_string();
                    response.verdict = Some(format!(
                        "Sandbox execution error on testcase {}: {}",
                        case_num + 1,
                        e
                    ));
                    break;
                }
            };

            let meta = result.meta;
            max_time = max_time.max(meta.time_ms);
            max_mem = max_mem.max(meta.memory_kb);

            // A. Check Time Limit Exceeded (Isolate status "TO" or CPU time exceeded)
            if meta.status.as_deref() == Some("TO")
                || meta.killed
                || meta.time_ms > payload.time_limit
            {
                response.status = "TLE".to_string();
                response.verdict =
                    Some(format!("Time limit exceeded on testcase {}", case_num + 1));
                break;
            }

            // B. Check Memory Limit Exceeded (Isolate status "ML" or RSS exceeded)
            let mem_limit_kb = (payload.memory_limit as i32) * 1024;
            if meta.status.as_deref() == Some("ML")
                || (payload.memory_limit > 0 && (meta.memory_kb as i32) > mem_limit_kb)
            {
                response.status = "MLE".to_string();
                response.verdict = Some(format!(
                    "Memory limit exceeded on testcase {}",
                    case_num + 1
                ));
                break;
            }

            // C. Check Runtime Error (Signal or non-zero exit code)
            if meta.status.as_deref() == Some("RE")
                || meta.status.as_deref() == Some("SG")
                || meta.exit_code != 0
            {
                let err_msg = if !result.stderr.trim().is_empty() {
                    result.stderr.chars().take(200).collect::<String>()
                } else {
                    meta.message.unwrap_or_else(|| {
                        format!("Process exited with status code {}", meta.exit_code)
                    })
                };

                response.status = "RE".to_string();
                response.verdict = Some(format!(
                    "Runtime error on testcase {}: {}",
                    case_num + 1,
                    err_msg
                ));
                break;
            }

            // D. Output Correctness Check
            let expected = Self::normalize_output(case.output.as_deref().unwrap_or(""));
            let actual = Self::normalize_output(&result.stdout);

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

        // 4. Cleanup isolate box
        let _ = IsolateSandbox::cleanup_box(core_id).await;

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

        tracing::debug!(
            submission_id = response.id,
            "Verdict published to RabbitMQ 'incomming' queue"
        );
        Ok(())
    }
}
