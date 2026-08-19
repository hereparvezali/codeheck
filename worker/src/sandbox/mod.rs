use std::{process::Output, sync::Arc};
use tokio::{io::AsyncWriteExt, process::Command};

use crate::{error::WorkerError, languages::LanguageStrategy};

pub struct DockerSandbox;

impl DockerSandbox {

    pub async fn build_compiler_images() -> Result<(), WorkerError> {
        let mut dir = tokio::fs::read_dir("./src/compilers").await?;
        while let Some(entry) = dir.next_entry().await? {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if name.starts_with("Dockerfile") {
                let tag = match name.split('.').nth(1) {
                    Some(t) => t,
                    None => continue,
                };
                let output = Command::new("docker").arg("images").output().await?;

                let got_tag = String::from_utf8_lossy(&output.stdout).contains(tag);
                if got_tag {
                    continue;
                }
                let child = Command::new("docker")
                    .args([
                        "buildx",
                        "build",
                        "--load",
                        "-f",
                        &path.display().to_string(),
                        "-t",
                        tag,
                        ".",
                    ])
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()?;

                if let Ok(output) = child.wait_with_output().await {
                    if !output.status.success() {
                        return Err(WorkerError::DockerBuild(format!(
                            "Failed to build image {}: {}",
                            tag,
                            String::from_utf8_lossy(&output.stderr)
                        )));
                    }
                    tracing::info!(image = %tag, "Built compiler sandbox image");
                }
            }
        }
        tracing::info!("All compiler and runtime sandbox images are ready");
        Ok(())
    }


    pub async fn compile(
        strategy: &Arc<dyn LanguageStrategy>,
        submission_id: i64,
        core_id: usize,
    ) -> Result<Option<Output>, WorkerError> {
        let job_dir_host = format!("/tmp/codebox/{}", submission_id);
        let job_dir_container = format!("/codebox/{}", submission_id);

        if let Some(compile_cmd) = strategy.compile_command(&job_dir_container) {
            let output = Command::new("docker")
                .args([
                    "run",
                    "--rm",
                    "-i",
                    "-v",
                    &format!("{}:{}", job_dir_host, job_dir_container),
                    "--cpus=1",
                    &format!("--cpuset-cpus={}", core_id),
                    "--memory=512m",
                    "--memory-swap=512m",
                    "--security-opt=no-new-privileges",
                    "--ulimit",
                    "nofile=64:64",
                    "--pids-limit=64",
                    "--network=none",
                    strategy.docker_image(),
                    "sh",
                    "-c",
                    &compile_cmd,
                ])
                .output()
                .await?;
            return Ok(Some(output));
        }
        Ok(None)
    }


    pub async fn execute_case(
        strategy: &Arc<dyn LanguageStrategy>,
        submission_id: i64,
        input: &Option<String>,
        time_limit_ms: i16,
        memory_limit_mb: i16,
        core_id: usize,
    ) -> Result<Output, WorkerError> {
        let job_dir_host = format!("/tmp/codebox/{}", submission_id);
        let job_dir_container = format!("/codebox/{}", submission_id);

        let run_cmd = strategy.run_command(&job_dir_container);
        let timeout_secs = (time_limit_ms as f32 / 1000.0).max(0.1);

        let execution_cmd = format!(
            "/usr/bin/time -v timeout -s 9 {}s {}",
            timeout_secs, run_cmd
        );

        let mut child = Command::new("docker")
            .args([
                "run",
                "--rm",
                "-i",
                "-v",
                &format!("{}:{}", job_dir_host, job_dir_container),
                "--cpus=1",
                &format!("--cpuset-cpus={}", core_id),
                &format!("--memory={}m", memory_limit_mb),
                &format!("--memory-swap={}m", memory_limit_mb),
                "--security-opt=no-new-privileges",
                "--ulimit",
                "nofile=64:64",
                "--pids-limit=64",
                "--network=none",
                strategy.docker_image(),
                "sh",
                "-c",
                &execution_cmd,
            ])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            if let Some(data) = input {
                stdin.write_all(data.as_bytes()).await?;
            }
        }

        Ok(child.wait_with_output().await?)
    }
}
