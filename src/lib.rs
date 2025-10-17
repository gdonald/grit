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
