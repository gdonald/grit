use std::fs;

#[test]
fn test_run_no_arguments() {
    let args = vec!["grit".to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 1);
}

#[test]
fn test_run_file_not_found() {
    let args = vec!["grit".to_string(), "nonexistent.grit".to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 1);
}

#[test]
fn test_run_valid_file() {
    use std::io::Write;

    // Create a temporary test file
    let test_file = "/tmp/test_run_valid_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"1 + 2").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Tokens:"));
    assert!(output_str.contains("Integer(1)"));
    assert!(output_str.contains("Plus"));
    assert!(output_str.contains("Integer(2)"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_empty_file() {
    use std::io::Write;

    // Create a temporary empty file
    let test_file = "/tmp/test_run_empty_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Tokens:"));
    assert!(output_str.contains("Eof"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_all_operators() {
    use std::io::Write;

    let test_file = "/tmp/test_run_operators_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"(1 + 2) * 3 / 4 - 5").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("LeftParen"));
    assert!(output_str.contains("Plus"));
    assert!(output_str.contains("Multiply"));
    assert!(output_str.contains("Divide"));
    assert!(output_str.contains("Minus"));
    assert!(output_str.contains("RightParen"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_parse_error_invalid_syntax() {
    use std::io::Write;

    let test_file = "/tmp/test_run_parse_error_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"(1 + 2").unwrap(); // Missing closing paren

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), 1);

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_with_ast_output() {
    use std::io::Write;

    let test_file = "/tmp/test_run_ast_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"1 + 2 * 3").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Tokens:"));
    assert!(output_str.contains("AST:"));
    assert!(output_str.contains("Debug AST:"));
    assert!(output_str.contains("BinaryOp"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_empty_input_message() {
    use std::io::Write;

    let test_file = "/tmp/test_run_empty_msg_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"   \n  \t  ").unwrap(); // Only whitespace

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Empty input - nothing to parse"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}
