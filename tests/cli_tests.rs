use std::fs;
use std::process::Command;

/// Helper function to get the path to the compiled binary
fn get_binary_path() -> String {
    // In debug mode, the binary is in target/debug/
    // In release mode, it's in target/release/
    let mut path = std::env::current_dir().unwrap();
    path.push("target");
    path.push("debug");
    path.push("grit");
    path.to_str().unwrap().to_string()
}

/// Helper function to create a temporary test file
fn create_test_file(filename: &str, content: &str) -> String {
    let path = format!("/tmp/{}", filename);
    fs::write(&path, content).expect("Failed to create test file");
    path
}

/// Helper function to clean up temporary test files
fn cleanup_test_file(path: &str) {
    let _ = fs::remove_file(path);
}

#[test]
fn test_cli_no_arguments() {
    let output = Command::new(&get_binary_path())
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Usage:"));
    assert!(stderr.contains("<file.grit>"));
}

#[test]
fn test_cli_file_not_found() {
    let output = Command::new(&get_binary_path())
        .arg("nonexistent_file.grit")
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error reading file"));
    assert!(stderr.contains("nonexistent_file.grit"));
}

#[test]
fn test_cli_simple_expression() {
    let test_file = create_test_file("test_simple.grit", "1 + 2");

    let output = Command::new(&get_binary_path())
        .arg(&test_file)
        .output()
        .expect("Failed to execute command");

    cleanup_test_file(&test_file);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Tokens:"));
    assert!(stdout.contains("Integer(1)"));
    assert!(stdout.contains("Plus"));
    assert!(stdout.contains("Integer(2)"));
    assert!(stdout.contains("Eof"));
}

#[test]
fn test_cli_complex_expression() {
    let test_file = create_test_file("test_complex.grit", "(10 + 20) * 3");

    let output = Command::new(&get_binary_path())
        .arg(&test_file)
        .output()
        .expect("Failed to execute command");

    cleanup_test_file(&test_file);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Tokens:"));
    assert!(stdout.contains("LeftParen"));
    assert!(stdout.contains("Integer(10)"));
    assert!(stdout.contains("Plus"));
    assert!(stdout.contains("Integer(20)"));
    assert!(stdout.contains("RightParen"));
    assert!(stdout.contains("Multiply"));
    assert!(stdout.contains("Integer(3)"));
}

#[test]
fn test_cli_empty_file() {
    let test_file = create_test_file("test_empty.grit", "");

    let output = Command::new(&get_binary_path())
        .arg(&test_file)
        .output()
        .expect("Failed to execute command");

    cleanup_test_file(&test_file);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Tokens:"));
    assert!(stdout.contains("Eof"));
}

#[test]
fn test_cli_multiline_expression() {
    let test_file = create_test_file("test_multiline.grit", "1 + 2\n3 * 4");

    let output = Command::new(&get_binary_path())
        .arg(&test_file)
        .output()
        .expect("Failed to execute command");

    cleanup_test_file(&test_file);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Integer(1)"));
    assert!(stdout.contains("Integer(2)"));
    assert!(stdout.contains("Integer(3)"));
    assert!(stdout.contains("Integer(4)"));
}

#[test]
fn test_cli_all_operators() {
    let test_file = create_test_file("test_operators.grit", "1 + 2 - 3 * 4 / 5");

    let output = Command::new(&get_binary_path())
        .arg(&test_file)
        .output()
        .expect("Failed to execute command");

    cleanup_test_file(&test_file);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Plus"));
    assert!(stdout.contains("Minus"));
    assert!(stdout.contains("Multiply"));
    assert!(stdout.contains("Divide"));
}

#[test]
fn test_cli_invalid_character() {
    let test_file = create_test_file("test_invalid.grit", "1 + @");

    let output = Command::new(&get_binary_path())
        .arg(&test_file)
        .output()
        .expect("Failed to execute command");

    cleanup_test_file(&test_file);

    // Should fail due to unexpected character
    assert!(!output.status.success());
}
