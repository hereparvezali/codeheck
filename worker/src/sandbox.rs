use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Output,
};
use tokio::{fs, process::Command};

use crate::{error::WorkerError, languages::LanguageStrategy};

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct IsolateMeta {
    pub time_ms: i16,
    pub wall_time_ms: i16,
    pub memory_kb: i16,
    pub status: Option<String>,
    pub message: Option<String>,
    pub exit_code: i32,
    pub killed: bool,
}

#[derive(Debug, Default, Clone)]
pub struct CaseExecutionResult {
    pub meta: IsolateMeta,
    pub stdout: String,
    pub stderr: String,
}

pub struct IsolateSandbox;

impl IsolateSandbox {
    /// Verifies that the isolate executable is present on the host
    pub async fn verify_environment() -> Result<(), WorkerError> {
        let output = Command::new("isolate")
            .arg("--version")
            .output()
            .await
            .map_err(|e| {
                WorkerError::Sandbox(format!(
                    "Isolate sandbox binary not found or not executable: {}",
                    e
                ))
            })?;

        if !output.status.success() {
            return Err(WorkerError::Sandbox(format!(
                "Isolate check failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let version_info = String::from_utf8_lossy(&output.stdout);
        let first_line = version_info.lines().next().unwrap_or("Isolate Sandbox");
        tracing::info!(info = %first_line, "Isolate sandbox environment verified");
        Ok(())
    }

    /// Initializes an isolate sandbox for a specific box_id (returns path to internal /box dir)
    pub async fn init_box(box_id: usize) -> Result<PathBuf, WorkerError> {
        let output = Command::new("isolate")
            .args(["--box-id", &box_id.to_string(), "--init"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(WorkerError::Sandbox(format!(
                "Failed to init isolate box {}: {}",
                box_id,
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let base_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let box_path = PathBuf::from(base_dir).join("box");

        if !fs::try_exists(&box_path).await.unwrap_or(false) {
            fs::create_dir_all(&box_path).await?;
        }

        Ok(box_path)
    }

    /// Cleans up the isolate box directory and any lingering resources
    pub async fn cleanup_box(box_id: usize) -> Result<(), WorkerError> {
        let _ = Command::new("isolate")
            .args(["--box-id", &box_id.to_string(), "--cleanup"])
            .output()
            .await;
        Ok(())
    }

    /// Compiles user code into the target box directory
    pub async fn compile(
        strategy: &dyn LanguageStrategy,
        src_path: &Path,
        out_path: &Path,
    ) -> Result<Option<Output>, WorkerError> {
        if let Some((program, args)) = strategy.compile_command(
            &src_path.display().to_string(),
            &out_path.display().to_string(),
        ) {
            let output = Command::new(&program)
                .args(&args)
                .output()
                .await
                .map_err(|e| {
                    WorkerError::Execution(format!(
                        "Failed to execute compiler '{}': {}",
                        program, e
                    ))
                })?;
            return Ok(Some(output));
        }
        Ok(None)
    }

    /// Executes a single test case within the Isolate sandbox
    pub async fn execute_case(
        strategy: &dyn LanguageStrategy,
        cpu_id: usize,
        box_dir: &Path,
        input: &Option<String>,
        time_limit_ms: i16,
        memory_limit_mb: i16,
    ) -> Result<CaseExecutionResult, WorkerError> {
        let stdin_file = box_dir.join("input.in");
        let stdout_file = box_dir.join("output.out");
        let stderr_file = box_dir.join("error.err");
        let meta_file = format!("/tmp/isolate_meta_{}.txt", cpu_id);

        if let Some(data) = input {
            fs::write(&stdin_file, data).await?;
        } else {
            fs::write(&stdin_file, "").await?;
        }

        let time_limit_sec = (time_limit_ms as f64 / 1000.0).max(0.05);
        let wall_time_sec = (time_limit_sec * 3.0 + 1.0).max(1.0);
        let mem_limit_kb = (memory_limit_mb as u32) * 1024;

        let mut args: Vec<String> = vec![
            "-c".to_string(),
            cpu_id.to_string(),
            "isolate".to_string(),
            format!("--box-id={}", cpu_id),
            format!("--time={:.3}", time_limit_sec),
            format!("--wall-time={:.3}", wall_time_sec),
            "--extra-time=0.5".to_string(),
            "--processes=64".to_string(),
            "--open-files=64".to_string(),
            format!("--meta={}", meta_file),
            "--stdin=input.in".to_string(),
            "--stdout=output.out".to_string(),
            "--stderr=error.err".to_string(),
            "--dir=/usr:maybe".to_string(),
            "--dir=/lib:maybe".to_string(),
            "--dir=/lib64:maybe".to_string(),
            "--dir=/etc:maybe".to_string(),
            "--dir=/bin:maybe".to_string(),
            "--dir=/dev:dev:maybe".to_string(),
        ];

        if strategy.supports_address_space_limit() {
            args.push(format!("--mem={}", mem_limit_kb));
        }

        args.push("--run".to_string());
        args.push("--".to_string());
        args.extend(strategy.run_command());

        let _ = Command::new("taskset").args(&args).output().await?;

        let meta = Self::parse_meta(&meta_file).await?;
        let stdout = fs::read_to_string(&stdout_file).await.unwrap_or_default();
        let stderr = fs::read_to_string(&stderr_file).await.unwrap_or_default();

        let _ = fs::remove_file(&meta_file).await;

        Ok(CaseExecutionResult {
            meta,
            stdout,
            stderr,
        })
    }

    async fn parse_meta(meta_path: &str) -> Result<IsolateMeta, WorkerError> {
        let content = fs::read_to_string(meta_path).await.unwrap_or_default();
        let mut map = HashMap::new();

        for line in content.lines() {
            if let Some((k, v)) = line.split_once(':') {
                map.insert(k.trim(), v.trim());
            }
        }

        let time_sec: f64 = map.get("time").and_then(|v| v.parse().ok()).unwrap_or(0.0);
        let time_ms = (time_sec * 1000.0).round() as i16;

        let wall_sec: f64 = map
            .get("time-wall")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0.0);
        let wall_time_ms = (wall_sec * 1000.0).round() as i16;

        let memory_kb: i16 = map
            .get("cg-mem")
            .or_else(|| map.get("max-rss"))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let exit_code: i32 = map
            .get("exitcode")
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let killed = map.get("killed").map(|v| *v == "1").unwrap_or(false);

        Ok(IsolateMeta {
            time_ms,
            wall_time_ms,
            memory_kb,
            status: map.get("status").map(|s| s.to_string()),
            message: map.get("message").map(|s| s.to_string()),
            exit_code,
            killed,
        })
    }
}
