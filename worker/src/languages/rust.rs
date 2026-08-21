use super::LanguageStrategy;

pub struct RustStrategy;

impl LanguageStrategy for RustStrategy {
    fn name(&self) -> &'static str {
        "Rust"
    }

    fn file_extension(&self) -> &'static str {
        "rs"
    }

    fn compile_command(&self, src_path: &str, out_path: &str) -> Option<(String, Vec<String>)> {
        Some((
            "rustc".to_string(),
            vec![
                "-O".to_string(),
                src_path.to_string(),
                "-o".to_string(),
                out_path.to_string(),
            ],
        ))
    }

    fn run_command(&self) -> Vec<String> {
        vec!["./Main".to_string()]
    }

    fn supports_address_space_limit(&self) -> bool {
        true
    }
}
