use worker::{
    languages::{cpp::CppStrategy, python::PythonStrategy},
    sandbox::IsolateSandbox,
};

#[tokio::test]
async fn test_sandbox_verification() {
    let res = IsolateSandbox::verify_environment().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_python_execution() {
    let strategy = PythonStrategy;
    let box_id = 99; // Use high box id to avoid conflicts
    let box_dir = IsolateSandbox::init_box(box_id).await.unwrap();

    let code = "a, b = map(int, input().split())\nprint(a + b)\n";
    tokio::fs::write(box_dir.join("Main.py"), code).await.unwrap();

    let input = Some("15 27\n".to_string());
    let result = IsolateSandbox::execute_case(&strategy, box_id, &box_dir, &input, 1000, 256)
        .await
        .unwrap();

    assert_eq!(result.stdout.trim(), "42");
    assert_eq!(result.meta.exit_code, 0);
    assert!(result.meta.time_ms < 500);

    IsolateSandbox::cleanup_box(box_id).await.unwrap();
}

#[tokio::test]
async fn test_cpp_compilation_and_execution() {
    let strategy = CppStrategy;
    let box_id = 98;
    let box_dir = IsolateSandbox::init_box(box_id).await.unwrap();

    let src_path = box_dir.join("Main.cpp");
    let out_path = box_dir.join("Main");

    let code = r#"
#include <iostream>
int main() {
    int x;
    if (std::cin >> x) {
        std::cout << (x * x) << "\n";
    }
    return 0;
}
"#;
    tokio::fs::write(&src_path, code).await.unwrap();

    let compile_output = IsolateSandbox::compile(&strategy, &src_path, &out_path)
        .await
        .unwrap();
    assert!(compile_output.is_some());
    assert!(compile_output.unwrap().status.success());

    let input = Some("12\n".to_string());
    let result = IsolateSandbox::execute_case(&strategy, box_id, &box_dir, &input, 1000, 256)
        .await
        .unwrap();

    assert_eq!(result.stdout.trim(), "144");
    assert_eq!(result.meta.exit_code, 0);

    IsolateSandbox::cleanup_box(box_id).await.unwrap();
}

#[tokio::test]
async fn test_tle_detection() {
    let strategy = PythonStrategy;
    let box_id = 97;
    let box_dir = IsolateSandbox::init_box(box_id).await.unwrap();

    let code = "while True:\n    pass\n";
    tokio::fs::write(box_dir.join("Main.py"), code).await.unwrap();

    let input = None;
    let result = IsolateSandbox::execute_case(&strategy, box_id, &box_dir, &input, 100, 128)
        .await
        .unwrap();

    assert!(
        result.meta.status.as_deref() == Some("TO")
            || result.meta.killed
            || result.meta.time_ms >= 100
    );

    IsolateSandbox::cleanup_box(box_id).await.unwrap();
}

#[tokio::test]
async fn test_re_detection() {
    let strategy = PythonStrategy;
    let box_id = 96;
    let box_dir = IsolateSandbox::init_box(box_id).await.unwrap();

    let code = "print(1 / 0)\n";
    tokio::fs::write(box_dir.join("Main.py"), code).await.unwrap();

    let input = None;
    let result = IsolateSandbox::execute_case(&strategy, box_id, &box_dir, &input, 1000, 128)
        .await
        .unwrap();

    assert!(
        result.meta.status.as_deref() == Some("RE")
            || result.meta.status.as_deref() == Some("SG")
            || result.meta.exit_code != 0
    );
    assert!(result.stderr.contains("ZeroDivisionError"));

    IsolateSandbox::cleanup_box(box_id).await.unwrap();
}
