/// Returns the file extension for a given language.
pub fn ext(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "cpp" | "c++" => "cpp",
        "python" | "py" => "py",
        "java" => "java",
        "rust" | "rs" => "rs",
        "go" => "go",
        "javascript" | "js" => "js",
        _ => "txt",
    }
}

/// Returns the Docker image for a given language.
pub fn image(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "cpp" | "c++" => "gcc-rebuilt:latest",
        "python" | "py" => "python:alpine",
        "java" => "openjdk-rebuilt:latest",
        "rust" | "rs" => "rust:alpine",
        "go" => "golang:alpine",
        "javascript" | "js" => "node:alpine",
        _ => "node:alpine",
    }
}

/// Returns the compile and run commands for a given language.
pub fn cmd(language: &str) -> (Option<String>, String) {
    match language.to_lowercase().as_str() {
        "cpp" | "c++" => (
            Some("g++ /Main/Main.cpp -o /Main/a.out".to_string()),
            "/Main/a.out".to_string(),
        ),
        "rust" => (
            Some("rustc /Main/Main.rs -o /Main/a.out".to_string()),
            "/Main/a.out".to_string(),
        ),
        "java" => (
            Some("javac /Main/Main.java".to_string()),
            "java -cp /Main Main".to_string(),
        ),
        "go" => (
            Some("go build -o /Main/a.out /Main/Main.go".to_string()),
            "/Main/a.out".to_string(),
        ),
        "python" | "py" => (None, "python /Main/Main.py".to_string()),
        "javascript" | "js" => (None, "node /Main/Main.js".to_string()),
        _ => (None, "cat /Main/Main".to_string()),
    }
}
