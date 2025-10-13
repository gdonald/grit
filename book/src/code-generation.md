# Code Generation

The `CodeGenerator` translates the parsed AST into Rust expressions and wraps them in a `fn main` scaffold.

```rust,ignore
use grit::parser::Parser;
use grit::lexer::Tokenizer;
use grit::codegen::CodeGenerator;

fn main() {
    let mut tokenizer = Tokenizer::new("1 + 2 * 3");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("valid program");

    let rust_program = CodeGenerator::generate_program(&ast);
    println!("{}", rust_program);
}
```

Output:

```text
fn main() {
    let result = 1 + 2 * 3;
    println!("{}", result);
}
```

### Precedence-aware rendering

The generator wraps sub-expressions in parentheses when needed to preserve semantics:

```rust,ignore
use grit::parser::{Expr, BinaryOperator};
use grit::codegen::CodeGenerator;

let expr = Expr::BinaryOp {
    left: Box::new(Expr::Integer(3)),
    op: BinaryOperator::Divide,
    right: Box::new(Expr::BinaryOp {
        left: Box::new(Expr::Integer(1)),
        op: BinaryOperator::Add,
        right: Box::new(Expr::Integer(2)),
    }),
};

assert_eq!(
    CodeGenerator::generate_expression(&expr),
    "3 / (1 + 2)"
);
```

The snippet above is executed as part of `mdbook test`, keeping documentation and code in sync.
