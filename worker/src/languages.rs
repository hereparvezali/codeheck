pub fn ext(language: &str) -> &str {
    match language {
        "cpp" | "c++" | "C++" => "cpp",
        "python" => "py",
        "java" => "java",
        "rust" | "Rust" => "rs",
        "Go" | "go" => "go",
        "javascript" | "js" => "js",
        _ => "txt",
    }
}
pub fn image(language: &str) -> &str {
    match language {
        "cpp" | "c++" | "C++" => "gcc:trixie",
        "python" | "py" => "python:alpine",
        "java" => "openjdk:slim",
        "rust" | "rs" => "rust:alpine",
        "go" | "Go" => "golang:alpine",
        "javascript" | "js" => "node:alpine",
        _ => "node:alpine",
    }
}
pub fn cmd(language: &str) -> (Option<String>, String) {
    match language.to_lowercase().as_str() {
        "cpp" | "c++" => (
            Some("g++ /code/code.cpp -o /code/a.out".to_string()),
            "/code/a.out".to_string(),
        ),
        "rust" => (
            Some("rustc /code/code.rs -o /code/a.out".to_string()),
            "/code/a.out".to_string(),
        ),
        "java" => (
            Some("javac /code/code.java".to_string()),
            "java -cp /code code".to_string(),
        ),
        "go" => (
            Some("go build -o /code/a.out /code/code.go".to_string()),
            "/code/a.out".to_string(),
        ),
        "python" | "py" => (None, "python3 /code/code.py".to_string()),
        "javascript" | "js" => (None, "node /code/code.js".to_string()),
        _ => (None, "cat /code/code".to_string()),
    }
}
