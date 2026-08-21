use super::LanguageStrategy;

pub struct JavaScriptStrategy;

impl LanguageStrategy for JavaScriptStrategy {
    fn name(&self) -> &'static str {
        "JavaScript"
    }

    fn file_extension(&self) -> &'static str {
        "js"
    }

    fn compile_command(&self, _src_path: &str, _out_path: &str) -> Option<(String, Vec<String>)> {
        None
    }

    fn run_command(&self) -> Vec<String> {
        vec!["/usr/bin/node".to_string(), "Main.js".to_string()]
    }

    fn supports_address_space_limit(&self) -> bool {
        false
    }
}
