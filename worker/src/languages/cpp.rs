use super::LanguageStrategy;

pub struct CppStrategy;

impl LanguageStrategy for CppStrategy {
    fn name(&self) -> &'static str {
        "C++"
    }

    fn file_extension(&self) -> &'static str {
        "cpp"
    }

    fn docker_image(&self) -> &'static str {
        "gcc-rebuilt:latest"
    }

    fn compile_command(&self, job_dir: &str) -> Option<String> {
        Some(format!(
            "g++ -O2 -std=c++20 {}/Main.cpp -o {}/Main",
            job_dir, job_dir
        ))
    }

    fn run_command(&self, job_dir: &str) -> String {
        format!("{}/Main", job_dir)
    }
}
