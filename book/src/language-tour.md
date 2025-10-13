# Language Tour

Grit currently focuses on arithmetic expressions. Source programs are tokenized, parsed into an Abstract Syntax Tree (AST), and then transpiled to Rust code.

## Example

```text
(10 + 20) * (30 - 15) / 5
```

Running the CLI:

```bash
cargo run -- examples/simple.grit
```

Produces:

```text
Tokens:
  LeftParen
  Integer(10)
  Plus
  Integer(20)
  RightParen
  Multiply
  LeftParen
  Integer(30)
  Minus
  Integer(15)
  RightParen
  Divide
  Integer(5)
  Eof

AST:
  ((10 + 20) * (30 - 15)) / 5
```

The parser performs precedence climbing so multiplication and division bind tighter than addition and subtraction. Parentheses override precedence, and expressions associate to the left.

## Next steps

- Try editing `examples/simple.grit` and rerunning the CLI.
- Look at the parser tests in `tests/parser_tests.rs` for more usage examples.
