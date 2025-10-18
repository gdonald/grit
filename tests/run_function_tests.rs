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
    assert!(output_str.contains("Generated Rust code:"));
    assert!(output_str.contains("fn main() {"));
    assert!(output_str.contains("println!(\"{}\", result);"));

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
    assert!(!output_str.contains("Generated Rust code"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_generated_rust_program_structure_integration() {
    use std::io::Write;

    let test_file = "/tmp/test_run_program_structure_integration.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"3 / (1 + 2)").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    let lines: Vec<&str> = output_str.lines().collect();
    let generated_index = lines
        .iter()
        .position(|line| line.trim() == "Generated Rust code:")
        .expect("Generated Rust code header missing");

    assert_eq!(lines[generated_index + 1], "  fn main() {");
    assert_eq!(
        lines[generated_index + 2],
        "      let result = 3 / (1 + 2);"
    );
    assert_eq!(
        lines[generated_index + 3],
        "      println!(\"{}\", result);"
    );
    assert_eq!(lines[generated_index + 4], "  }");

    // Cleanup
    let _ = fs::remove_file(test_file);
}

// Tests for function definitions and calls

#[test]
fn test_run_function_definition() {
    use std::io::Write;

    let test_file = "/tmp/test_run_function_def.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"fn add(a, b) { a + b }").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Tokens:"));
    assert!(output_str.contains("Fn"));
    assert!(output_str.contains("AST:"));
    assert!(output_str.contains("FunctionDef"));
    assert!(output_str.contains("Generated Rust code:"));
    assert!(output_str.contains("fn add("));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_function_call() {
    use std::io::Write;

    let test_file = "/tmp/test_run_function_call.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"print(42)").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Identifier"));
    assert!(output_str.contains("FunctionCall"));
    assert!(output_str.contains("Generated Rust code:"));
    assert!(output_str.contains("println!"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_function_with_multiple_params() {
    use std::io::Write;

    let test_file = "/tmp/test_run_function_params.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"fn calculate(x, y, z) { x + y * z }")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("FunctionDef"));
    assert!(output_str.contains("calculate"));
    assert!(output_str.contains("fn calculate("));
    assert!(output_str.contains("x,"));
    assert!(output_str.contains("y,"));
    assert!(output_str.contains("z"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

// Tests for assignments and statements

#[test]
fn test_run_assignment_statement() {
    use std::io::Write;

    let test_file = "/tmp/test_run_assignment.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"x = 42").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("Generated Rust code:"));
    assert!(output_str.contains("let x = 42"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_assignment_with_expression() {
    use std::io::Write;

    let test_file = "/tmp/test_run_assignment_expr.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"result = 10 + 20 * 3").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("BinaryOp"));
    assert!(output_str.contains("let result ="));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_multiple_assignments() {
    use std::io::Write;

    let test_file = "/tmp/test_run_multiple_assignments.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"x = 10\ny = 20\nz = x + y").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("let x = 10"));
    assert!(output_str.contains("let y = 20"));
    assert!(output_str.contains("let z = x + y"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

// Tests for string literals

#[test]
fn test_run_string_literal() {
    use std::io::Write;

    let test_file = "/tmp/test_run_string.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"print('Hello, World!')").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("String"));
    assert!(output_str.contains("Hello, World!"));
    assert!(output_str.contains("println!"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_string_assignment() {
    use std::io::Write;

    let test_file = "/tmp/test_run_string_assign.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"message = 'Grit Language'").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("String"));
    assert!(output_str.contains("Grit Language"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

// Tests for control flow

#[test]
fn test_run_if_statement() {
    use std::io::Write;

    let test_file = "/tmp/test_run_if.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"if x > 5 { print(x) }").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("If"));
    assert!(output_str.contains("GreaterThan"));
    assert!(output_str.contains("if x > 5"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_if_else_statement() {
    use std::io::Write;

    let test_file = "/tmp/test_run_if_else.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"if x == 0 { print('zero') } else { print('non-zero') }")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("If"));
    assert!(output_str.contains("EqualEqual"));
    assert!(output_str.contains("if x == 0"));
    assert!(output_str.contains("else"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_if_elif_else_statement() {
    use std::io::Write;

    let test_file = "/tmp/test_run_if_elif_else.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(
        b"if x < 0 { print('negative') } elif x == 0 { print('zero') } else { print('positive') }",
    )
    .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("If"));
    assert!(output_str.contains("LessThan"));
    assert!(output_str.contains("EqualEqual"));
    assert!(output_str.contains("else if"));
    assert!(output_str.contains("else"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_while_loop() {
    use std::io::Write;

    let test_file = "/tmp/test_run_while.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"while count < 10 { count = count + 1 }")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("While"));
    assert!(output_str.contains("LessThan"));
    assert!(output_str.contains("while count < 10"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

// Tests for classes and methods

#[test]
fn test_run_class_definition() {
    use std::io::Write;

    let test_file = "/tmp/test_run_class.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"class Point").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Class"));
    assert!(output_str.contains("ClassDef"));
    assert!(output_str.contains("Point"));
    assert!(output_str.contains("struct Point"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_method_definition() {
    use std::io::Write;

    let test_file = "/tmp/test_run_method.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"fn Point > new(x, y) { self.x = x\nself.y = y }")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("MethodDef"));
    assert!(output_str.contains("Point"));
    assert!(output_str.contains("new"));
    assert!(output_str.contains("impl Point"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_field_access() {
    use std::io::Write;

    let test_file = "/tmp/test_run_field_access.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"print(point.x)").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    // Note: field access is parsed as method call with no args
    assert!(output_str.contains("MethodCall"));
    assert!(output_str.contains("point"));
    assert!(output_str.contains("point.x"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_method_call() {
    use std::io::Write;

    let test_file = "/tmp/test_run_method_call.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"result = point.distance(other)").unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("MethodCall"));
    assert!(output_str.contains("distance"));
    assert!(output_str.contains("point.distance"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

// Tests for complex programs

#[test]
fn test_run_complete_program_with_function_and_call() {
    use std::io::Write;

    let test_file = "/tmp/test_run_complete_program.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"fn square(n) { n * n }\nresult = square(5)\nprint(result)")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("FunctionDef"));
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("FunctionCall"));
    assert!(output_str.contains("square"));
    assert!(output_str.contains("fn square("));
    assert!(output_str.contains("let result"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_program_with_class_and_method() {
    use std::io::Write;

    let test_file = "/tmp/test_run_class_method_program.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"class Counter\nfn Counter > increment() { self.count = self.count + 1 }")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("ClassDef"));
    assert!(output_str.contains("MethodDef"));
    assert!(output_str.contains("Counter"));
    assert!(output_str.contains("increment"));
    assert!(output_str.contains("struct Counter"));
    assert!(output_str.contains("impl Counter"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_program_with_control_flow_and_assignments() {
    use std::io::Write;

    let test_file = "/tmp/test_run_control_flow_program.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"x = 10\nif x > 5 { y = x * 2 } else { y = x / 2 }\nprint(y)")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("If"));
    assert!(output_str.contains("GreaterThan"));
    assert!(output_str.contains("let x = 10"));
    assert!(output_str.contains("if x > 5"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_program_with_nested_expressions() {
    use std::io::Write;

    let test_file = "/tmp/test_run_nested_expr.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"result = ((1 + 2) * (3 + 4)) / (5 - 2)")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("Assignment"));
    assert!(output_str.contains("BinaryOp"));
    assert!(output_str.contains("Grouped"));
    assert!(output_str.contains("let result ="));
    assert!(output_str.contains("(1 + 2)"));
    assert!(output_str.contains("(3 + 4)"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}

#[test]
fn test_run_program_with_comparison_operators() {
    use std::io::Write;

    let test_file = "/tmp/test_run_comparisons.grit";
    let mut file = fs::File::create(test_file).unwrap();
    file.write_all(b"if x >= 10 { print('high') } elif x <= 5 { print('low') }")
        .unwrap();

    let args = vec!["grit".to_string(), test_file.to_string()];
    let mut output = Vec::new();

    let result = grit::run(&args, &mut output);
    assert!(result.is_ok());

    let output_str = String::from_utf8(output).unwrap();
    assert!(output_str.contains("GreaterThanOrEqual"));
    assert!(output_str.contains("LessThanOrEqual"));
    assert!(output_str.contains("if x >= 10"));
    assert!(output_str.contains("else if x <= 5"));

    // Cleanup
    let _ = fs::remove_file(test_file);
}
