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
        "python" | "py" => "python-rebuilt:latest",
        "java" => "openjdk-rebuilt:latest",
        "rust" | "rs" => "rust-rebuilt:latest",
        "go" => "golang-rebuilt:latest",
        "javascript" | "js" => "node-rebuilt:latest",
        _ => "node-rebuilt:latest",
    }
}

/// Returns the compile and run commands for a given language.
pub fn cmd(language: &str, id: &i64) -> (Option<String>, String) {
    let job_dir = format!("/codebox/{}", id); // Unique directory for each job
    match language.to_lowercase().as_str() {
        "cpp" | "c++" => (
            Some(format!("g++ {}/Main.cpp -o {}/Main", job_dir, job_dir)),
            format!("{}/Main", job_dir),
        ),
        "rust" => (
            Some(format!("rustc {}/Main.rs -o {}/Main", job_dir, job_dir)),
            format!("{}/Main", job_dir),
        ),
        "java" => (
            Some(format!("javac {}/Main.java -d {}", job_dir, job_dir)), // Compile to the unique directory
            format!("java -cp {} Main", job_dir), // Run from the unique directory
        ),
        "go" => (
            Some(format!("go build -o {}/Main {}/Main.go", job_dir, job_dir)),
            format!("{}/Main", job_dir),
        ),
        "python" | "py" => (None, format!("python {}/Main.py", job_dir)),
        "javascript" | "js" => (None, format!("node {}/Main.js", job_dir)),
        _ => (None, format!("cat {}/Main", job_dir)),
    }
}
