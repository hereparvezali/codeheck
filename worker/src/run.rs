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
) -> Result<String, anyhow::Error> {
    let (compile_cmd, run_cmd) = cmd(language);
    let time_sec = time_limit_ms as f64 / 1000.0;

    // Use /usr/bin/time to measure time and memory
    // Output format: "<elapsed_seconds> <max_memory_kb>"
    let measure_cmd = format!("/usr/bin/time -f '%e %M' sh -c \"{}\"", run_cmd);

    let mut full_cmd = String::new();
    if let Some(ref compile) = compile_cmd {
        full_cmd.push_str(&format!("{} && ", compile));
    }
    full_cmd.push_str(&format!("timeout -s 9 {}s {}", time_sec, measure_cmd));

    let mut child = tokio::process::Command::new("docker")
        .args([
            "run",
            "--rm",
            "-i",
            // "--read-only",
            // "--user",
            // "nobody",
            "-v",
            &format!("{}:/Main/Main.{}", code_path, ext(language)),
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
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        if let Some(data) = input {
            stdin.write_all(data.as_bytes()).await?;
        }
    }

    let output = child.wait_with_output().await?;

    // Parse /usr/bin/time output from stderr
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut time_used = None;
    let mut memory_used = None;
    for line in stderr.lines() {
        // Try to parse lines like: "<elapsed_seconds> <max_memory_kb>"
        if let Some((t, m)) = line.split_once(' ') {
            if let Ok(t) = t.trim().parse::<f64>() {
                time_used = Some((t * 1000.0) as i16); // ms
            }
            if let Ok(m) = m.trim().parse::<i16>() {
                memory_used = Some(m); // KB
            }
        }
    }
    response.time = time_used;
    response.memory = memory_used;

    if output.status.code() == Some(124) {
        response.status = "TLE".to_string();
        response.verdict = Some(stderr.to_string())
    } else if !output.status.success() {
        if compile_cmd.is_some() {
            response.status = "CE/RE".to_string();
        } else {
            response.status = "RE".to_string();
        }
        response.verdict = Some(stderr.to_string())
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
