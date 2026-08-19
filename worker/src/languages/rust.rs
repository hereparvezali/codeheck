use super::LanguageStrategy;

pub struct RustStrategy;

impl LanguageStrategy for RustStrategy {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn file_extension(&self) -> &'static str {
        "rs"
    }

    fn docker_image(&self) -> &'static str {
        "rust-rebuilt:latest"
    }

    fn compile_command(&self, job_dir: &str) -> Option<String> {
        Some(format!(
            "rustc -O {}/Main.rs -o {}/Main",
            job_dir, job_dir
        ))
    }

    fn run_command(&self, job_dir: &str) -> String {
        format!("{}/Main", job_dir)
    }
}
