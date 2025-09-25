use tokio::{fs, process::Command};

pub async fn build() {
    let mut dir = fs::read_dir("worker").await.unwrap();
    while let Some(entry) = dir.next_entry().await.unwrap() {
        let path = entry.path();
        let name = path.file_name().unwrap();
        if name.to_string_lossy().starts_with("Dockerfile") {
            let tag = name.to_string_lossy();
            Command::new("docker")
                .args([
                    "build",
                    "-f",
                    &format!("{}", path.display()),
                    "-t",
                    &format!("{}", tag.split(".").nth(1).unwrap()),
                    "worker",
                ])
                .status()
                .await
                .unwrap();
        }
    }
}
