use super::LanguageStrategy;

pub struct GoStrategy;

impl LanguageStrategy for GoStrategy {
    fn name(&self) -> &'static str {
        "Go"
    }

    fn file_extension(&self) -> &'static str {
        "go"
    }

    fn docker_image(&self) -> &'static str {
        "go-rebuilt:latest"
    }

    fn compile_command(&self, job_dir: &str) -> Option<String> {
        Some(format!("go build -o {}/Main {}/Main.go", job_dir, job_dir))
    }

    fn run_command(&self, job_dir: &str) -> String {
        format!("{}/Main", job_dir)
    }
}
