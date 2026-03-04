use crate::{
    language::{cmd, image},
    models::SubmissionPublishQueue,
};
use std::process::Output;
use tokio::{fs, process::Command};

/// Builds all Docker images defined in the worker directory.
pub async fn build_images() -> anyhow::Result<(), anyhow::Error> {
    let mut dir = tokio::fs::read_dir("./src/compilers").await?;
    while let Some(entry) = dir.next_entry().await? {
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if name.starts_with("Dockerfile") {
            let tag = name.split('.').nth(1).unwrap();
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
                    return Err(anyhow::anyhow!(
                        "Failed to build image: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
                println!("{} built successfully", tag.split('.').nth(1).unwrap());
            }
        }
    }
    println!("Compiler images are built");
    Ok(())
}

pub async fn run(
    input: &Option<String>,
    payload: &SubmissionPublishQueue,
    core_id: usize,
) -> anyhow::Result<Output, anyhow::Error> {
    let job_dir = format!("/tmp/codebox/{}", payload.submission_id);
    let (compile_cmd, run_cmd) = cmd(&payload.language, &payload.submission_id);

    let execution_cmd = format!(
        "/usr/bin/time -v timeout -s 9 {}s {}", // -v for getting max memory usage
        payload.time_limit as f32 / 1000.0,
        run_cmd
    );

    let mut full_cmd = String::new();
    if let Some(ref compile) = compile_cmd
        && !fs::try_exists(format!("/tmp/codebox/{}/Main", payload.submission_id))
            .await
            .unwrap()
    {
        full_cmd.push_str(&format!("{} && ", compile));
    }
    full_cmd.push_str(&execution_cmd);

    let mut child = Command::new("docker")
        .args([
            "run",
            "--rm",
            "-i",
            "-v",
            &format!("{}:/codebox/{}", job_dir, payload.submission_id), // Mount the unique directory
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
