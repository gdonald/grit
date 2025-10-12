# Grit - Generated Rust Intermediate Translation

![CI](https://github.com/gdonald/grit/workflows/CI/badge.svg)

A scripting language that transpiles to Rust source code, which then compiles to a Rust binary.

## Features

Currently, Grit supports:
- Integer literals
- Arithmetic operators: `+`, `-`, `*`, `/`
- Parentheses for grouping expressions

## Project Structure

```
grit/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── lib.rs            # Library root
│   └── lexer/            # Lexical analysis (tokenization)
│       ├── mod.rs        # Lexer module
│       ├── token.rs      # Token types and definitions
│       └── tokenizer.rs  # Tokenizer implementation
├── tests/                # Integration tests (separate from implementation)
│   ├── tokenizer_tests.rs # Tokenizer functionality tests
│   ├── token_tests.rs     # Token type tests
│   └── position_tests.rs  # Position tracking tests
├── examples/             # Example Grit programs
│   └── simple.grit       # Simple arithmetic example
├── .github/
│   └── workflows/
│       └── ci.yml        # GitHub Actions CI workflow
├── Cargo.toml            # Rust package manifest
└── README.md             # This file
```

## Building

```bash
cargo build
```

## Running Tests

All tests are located in separate files under the `tests/` directory:

```bash
cargo test
```

Run specific test modules:

```bash
cargo test --test tokenizer_tests
cargo test --test token_tests
cargo test --test position_tests
```

## Usage

```bash
cargo run -- examples/simple.grit
```

This will tokenize the input file and display the tokens.

## Example

Given a file `examples/simple.grit`:

```
(10 + 20) * (30 - 15) / 5
```

Running the tokenizer:

```bash
cargo run -- examples/simple.grit
```

Output:

```
Tokens:
  Token { token_type: LeftParen, line: 1, column: 1 }
  Token { token_type: Integer(10), line: 1, column: 2 }
  Token { token_type: Plus, line: 1, column: 5 }
  Token { token_type: Integer(20), line: 1, column: 7 }
  Token { token_type: RightParen, line: 1, column: 9 }
  Token { token_type: Multiply, line: 1, column: 11 }
  Token { token_type: LeftParen, line: 1, column: 13 }
  Token { token_type: Integer(30), line: 1, column: 14 }
  Token { token_type: Minus, line: 1, column: 17 }
  Token { token_type: Integer(15), line: 1, column: 19 }
  Token { token_type: RightParen, line: 1, column: 21 }
  Token { token_type: Divide, line: 1, column: 23 }
  Token { token_type: Integer(5), line: 1, column: 25 }
  Token { token_type: Eof, line: 1, column: 26 }
```

## Continuous Integration

The project uses GitHub Actions for continuous integration. On every push and pull request to the `main` branch, the workflow will:

- Run on Ubuntu with stable Rust
- Check code formatting with `rustfmt`
- Run linting with `clippy`
- Build the project
- Run all tests
- Generate and upload code coverage reports to Codecov

To ensure your code passes CI checks before pushing:

```bash
cargo fmt -- --check    # Check formatting
cargo clippy -- -D warnings  # Run linter
cargo test              # Run all tests
```

## Development Roadmap

- [x] Tokenizer with integers, operators, and parentheses
- [x] GitHub Actions CI workflow
- [ ] Parser for building an Abstract Syntax Tree (AST)
- [ ] AST to Rust code generator
- [ ] Support for variables
- [ ] Support for functions
- [ ] Support for control flow (if/else, loops)
- [ ] Type system
- [ ] Standard library

## License

MIT
