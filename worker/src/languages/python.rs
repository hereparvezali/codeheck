use super::LanguageStrategy;

pub struct PythonStrategy;

impl LanguageStrategy for PythonStrategy {
    fn name(&self) -> &'static str {
        "Python"
    }

    fn file_extension(&self) -> &'static str {
        "py"
    }

    fn compile_command(&self, _src_path: &str, _out_path: &str) -> Option<(String, Vec<String>)> {
        None
    }

    fn run_command(&self) -> Vec<String> {
        vec!["/usr/bin/python3".to_string(), "Main.py".to_string()]
    }

    fn supports_address_space_limit(&self) -> bool {
        true
    }
}
