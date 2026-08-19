use super::LanguageStrategy;

pub struct PythonStrategy;

impl LanguageStrategy for PythonStrategy {
    fn name(&self) -> &'static str {
        "Python"
    }

    fn file_extension(&self) -> &'static str {
        "py"
    }

    fn docker_image(&self) -> &'static str {
        "python-rebuilt:latest"
    }

    fn compile_command(&self, _job_dir: &str) -> Option<String> {
        None
    }

    fn run_command(&self, job_dir: &str) -> String {
        format!("python3 {}/Main.py", job_dir)
    }
}
