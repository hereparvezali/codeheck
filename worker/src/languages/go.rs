use super::LanguageStrategy;

pub struct GoStrategy;

impl LanguageStrategy for GoStrategy {
    fn name(&self) -> &'static str {
        "Go"
    }

    fn file_extension(&self) -> &'static str {
        "go"
    }

    fn compile_command(&self, src_path: &str, out_path: &str) -> Option<(String, Vec<String>)> {
        Some((
            "go".to_string(),
            vec![
                "build".to_string(),
                "-o".to_string(),
                out_path.to_string(),
                src_path.to_string(),
            ],
        ))
    }

    fn run_command(&self) -> Vec<String> {
        vec!["./Main".to_string()]
    }

    fn supports_address_space_limit(&self) -> bool {
        false
    }
}
