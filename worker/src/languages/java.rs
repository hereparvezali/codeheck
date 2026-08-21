use super::LanguageStrategy;

pub struct JavaStrategy;

impl LanguageStrategy for JavaStrategy {
    fn name(&self) -> &'static str {
        "Java"
    }

    fn file_extension(&self) -> &'static str {
        "java"
    }

    fn compile_command(&self, src_path: &str, out_path: &str) -> Option<(String, Vec<String>)> {
        Some((
            "javac".to_string(),
            vec![
                src_path.to_string(),
                "-d".to_string(),
                out_path.to_string(),
            ],
        ))
    }

    fn run_command(&self) -> Vec<String> {
        vec![
            "java".to_string(),
            "-Xmx512m".to_string(),
            "-cp".to_string(),
            ".".to_string(),
            "Main".to_string(),
        ]
    }

    fn supports_address_space_limit(&self) -> bool {
        false
    }
}
