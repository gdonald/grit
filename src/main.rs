use grit::lexer::Tokenizer;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

/// Run the tokenizer on the given arguments and write output to the given writer
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

    let mut tokenizer = Tokenizer::new(&source);
    let tokens = tokenizer.tokenize();

    writeln!(output, "Tokens:").unwrap();
    for token in tokens {
        writeln!(output, "  {:?}", token).unwrap();
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdout = io::stdout();

    if let Err(code) = run(&args, &mut stdout) {
        process::exit(code);
    }
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
        let test_file = "/tmp/test_run_valid.grit";
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
        let test_file = "/tmp/test_run_empty.grit";
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

        let test_file = "/tmp/test_run_operators.grit";
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
}
