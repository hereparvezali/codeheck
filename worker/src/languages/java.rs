use super::LanguageStrategy;

pub struct JavaStrategy;

impl LanguageStrategy for JavaStrategy {
    fn name(&self) -> &'static str {
        "Java"
    }

    fn file_extension(&self) -> &'static str {
        "java"
    }

    fn docker_image(&self) -> &'static str {
        "openjdk-rebuilt:latest"
    }

    fn compile_command(&self, job_dir: &str) -> Option<String> {
        Some(format!("javac {}/Main.java -d {}", job_dir, job_dir))
    }

    fn run_command(&self, job_dir: &str) -> String {
        format!("java -cp {} Main", job_dir)
    }
}
