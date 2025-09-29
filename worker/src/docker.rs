use crate::{
    language::{cmd, image},
    models::SubmissionPublishQueue,
};
use std::process::Output;
use tokio::process::Command;

/// Builds all Docker images defined in the worker directory.
pub async fn build_images() -> anyhow::Result<(), anyhow::Error> {
    let mut dir = tokio::fs::read_dir("worker").await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let name = path.file_name().unwrap();
        if name.to_string_lossy().starts_with("Dockerfile") {
            let tag = name.to_string_lossy();
            Command::new("docker")
                .args([
                    "build",
                    "-f",
                    &format!("{}", path.display()),
                    "-t",
                    &format!("{}", tag.split('.').nth(1).unwrap()),
                    "worker",
                ])
                .status()
                .await?;
        }
    }
    Ok(())
}

pub async fn run(
    code_path: &str,
    input: &Option<String>,
    payload: &SubmissionPublishQueue,
    core_id: usize,
) -> anyhow::Result<Output, anyhow::Error> {
    let (compile_cmd, run_cmd) = cmd(&payload.language);

    let execution_cmd = format!(
        "/usr/bin/time -v timeout -s 9 {}s {}", // -v for getting max memory usage
        payload.time_limit as f32 / 1000.0,
        run_cmd
    );

    let mut full_cmd = String::new();
    if let Some(ref compile) = compile_cmd {
        full_cmd.push_str(&format!("{} && ", compile));
    }
    full_cmd.push_str(&format!("{}", execution_cmd));

    let mut child = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-i",
            "-v",
            &format!(
                "{}:/Main/Main.{}",
                code_path,
                crate::language::ext(&payload.language)
            ),
            "--cpus=1",
            &format!("--cpuset-cpus={}", core_id),
            &format!("--memory={}m", payload.memory_limit),
            "--network=none",
            image(&payload.language),
            "sh",
            "-c",
            &full_cmd,
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        if let Some(data) = input {
            tokio::io::AsyncWriteExt::write_all(&mut stdin, data.as_bytes()).await?;
        }
    }

    anyhow::Ok(
        child
            .wait_with_output()
            .await
            .map_err(|e| anyhow::Error::new(e))?,
    )
}
