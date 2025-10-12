use grit::lexer::Tokenizer;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <file.grit>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        process::exit(1);
    });

    let mut tokenizer = Tokenizer::new(&source);
    let tokens = tokenizer.tokenize();

    println!("Tokens:");
    for token in tokens {
        println!("  {:?}", token);
    }
}
