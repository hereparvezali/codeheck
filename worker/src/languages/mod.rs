pub mod cpp;
pub mod go;
pub mod java;
pub mod javascript;
pub mod python;
pub mod rust;

use std::sync::Arc;

pub trait LanguageStrategy: Send + Sync {
    #[allow(dead_code)]
    fn name(&self) -> &'static str;
    fn file_extension(&self) -> &'static str;
    fn docker_image(&self) -> &'static str;
    fn compile_command(&self, job_dir: &str) -> Option<String>;
    fn run_command(&self, job_dir: &str) -> String;
}

pub struct LanguageRegistry;

impl LanguageRegistry {
    pub fn get(language: &str) -> Option<Arc<dyn LanguageStrategy>> {
        match language.to_lowercase().as_str() {
            "cpp" | "c++" => Some(Arc::new(cpp::CppStrategy)),
            "python" | "py" => Some(Arc::new(python::PythonStrategy)),
            "rust" | "rs" => Some(Arc::new(rust::RustStrategy)),
            "java" => Some(Arc::new(java::JavaStrategy)),
            "go" => Some(Arc::new(go::GoStrategy)),
            "javascript" | "js" => Some(Arc::new(javascript::JavaScriptStrategy)),
            _ => None,
        }
    }
}
