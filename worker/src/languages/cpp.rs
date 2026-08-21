use super::LanguageStrategy;

pub struct CppStrategy;

impl LanguageStrategy for CppStrategy {
    fn name(&self) -> &'static str {
        "C++"
    }

    fn file_extension(&self) -> &'static str {
        "cpp"
    }

    fn compile_command(&self, src_path: &str, out_path: &str) -> Option<(String, Vec<String>)> {
        Some((
            "g++".to_string(),
            vec![
                "-O2".to_string(),
                "-std=c++20".to_string(),
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
