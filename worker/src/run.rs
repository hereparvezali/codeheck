use std::process::Stdio;
use tokio::io::AsyncWriteExt as _;

use crate::{
    languages::{cmd, ext},
    types::ResponseFromWorker,
};

pub async fn run_in_docker(
    image: &str,
    code_path: &str,
    input: &Option<String>,
    time_limit_ms: i16,
    memory_limit: i16,
    language: &str,
    core_id: usize,
    response: &mut ResponseFromWorker,
) -> String {
    let (compile_cmd, run_cmd) = cmd(language);
    let time_sec = time_limit_ms as f64 / 1000.0;

    let mut full_cmd = String::new();
    if let Some(ref compile) = compile_cmd {
        full_cmd.push_str(&format!("{} && ", compile));
    }
    full_cmd.push_str(&format!("timeout -s 9 {}s {}", time_sec, run_cmd));

    let mut child = tokio::process::Command::new("docker")
        .args([
            "run",
            "--rm",
            "-i",
            "-v",
            &format!("{}:/code/code.{}", code_path, ext(language)),
            "--cpus=1",
            &format!("--memory={}m", memory_limit),
            "--network=none",
            &format!("--cpuset-cpus={}", core_id.to_string()),
            image,
            "sh",
            "-c",
            &full_cmd,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn docker");

    if let Some(mut stdin) = child.stdin.take() {
        if let Some(data) = input {
            stdin.write_all(data.as_bytes()).await.unwrap();
        }
    }

    let output = child.wait_with_output().await.unwrap();

    if output.status.code() == Some(124) {
        response.status = "TLE".to_string();
        response.verdict = Some(String::from_utf8_lossy(&output.stderr).to_string())
    } else if !output.status.success() {
        if compile_cmd.is_some() {
            response.status = "CE/RE".to_string();
        } else {
            response.status = "RE".to_string();
        }
        response.verdict = Some(String::from_utf8_lossy(&output.stderr).to_string())
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}
