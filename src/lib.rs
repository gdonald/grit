pub mod codegen;
pub mod lexer;
pub mod parser;

use codegen::CodeGenerator;
use lexer::Tokenizer;
use parser::Parser;
use std::fs;
use std::io::Write;

/// Run the tokenizer and parser on the given arguments and write output to the given writer
/// Returns Ok(()) on success, Err with exit code on failure
pub fn run<W: Write>(args: &[String], output: &mut W) -> Result<(), i32> {
    if args.len() < 2 {
        eprintln!("Usage: {} <file.grit>", args[0]);
        return Err(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).map_err(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        1
    })?;

    // Tokenize
    let mut tokenizer = Tokenizer::new(&source);
    let tokens = tokenizer.tokenize();

    writeln!(output, "Tokens:").unwrap();
    for token in &tokens {
        writeln!(output, "  {:?}", token).unwrap();
    }
    writeln!(output).unwrap();

    // Parse (skip if input is empty)
    if source.trim().is_empty() {
        writeln!(output, "Empty input - nothing to parse").unwrap();
    } else {
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(program) => {
                writeln!(output, "AST:").unwrap();
                writeln!(output, "  {}", program).unwrap();
                writeln!(output).unwrap();
                writeln!(output, "Debug AST:").unwrap();
                writeln!(output, "  {:?}", program).unwrap();
                writeln!(output).unwrap();

                // Generate Rust code
                let rust_code = CodeGenerator::generate_program(&program);
                writeln!(output, "Generated Rust code:").unwrap();
                for line in rust_code.trim_end().lines() {
                    writeln!(output, "  {}", line).unwrap();
                }
            }
            Err(err) => {
                eprintln!("Parse error: {}", err);
                return Err(1);
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_no_arguments() {
        let args = vec!["grit".to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 1);
    }

    #[test]
    fn test_run_file_not_found() {
        let args = vec!["grit".to_string(), "nonexistent.grit".to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 1);
    }

    #[test]
    fn test_run_valid_file() {
        use std::io::Write;

        // Create a temporary test file
        let test_file = "/tmp/test_run_valid_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"1 + 2").unwrap();

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
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
        let test_file = "/tmp/test_run_empty_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"").unwrap();

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
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

        let test_file = "/tmp/test_run_operators_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"(1 + 2) * 3 / 4 - 5").unwrap();

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
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

        let test_file = "/tmp/test_run_parse_error_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"(1 + 2").unwrap(); // Missing closing paren

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), 1);

        // Cleanup
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_run_with_ast_output() {
        use std::io::Write;

        let test_file = "/tmp/test_run_ast_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"1 + 2 * 3").unwrap();

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
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

        let test_file = "/tmp/test_run_empty_msg_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"   \n  \t  ").unwrap(); // Only whitespace

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
        assert!(result.is_ok());

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Empty input - nothing to parse"));
        assert!(!output_str.contains("Generated Rust code"));

        // Cleanup
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_run_generated_rust_program_structure() {
        use std::io::Write;

        let test_file = "/tmp/test_run_program_structure_lib.grit";
        let mut file = fs::File::create(test_file).unwrap();
        file.write_all(b"3 / (1 + 2)").unwrap();

        let args = vec!["grit".to_string(), test_file.to_string()];
        let mut output = Vec::new();

        let result = run(&args, &mut output);
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
}
