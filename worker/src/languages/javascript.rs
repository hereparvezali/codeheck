use super::LanguageStrategy;

pub struct JavaScriptStrategy;

impl LanguageStrategy for JavaScriptStrategy {
    fn name(&self) -> &'static str {
        "JavaScript"
    }

    fn file_extension(&self) -> &'static str {
        "js"
    }

    fn docker_image(&self) -> &'static str {
        "js-rebuilt:latest"
    }

    fn compile_command(&self, _job_dir: &str) -> Option<String> {
        None
    }

    fn run_command(&self, job_dir: &str) -> String {
        format!("node {}/Main.js", job_dir)
    }
}
