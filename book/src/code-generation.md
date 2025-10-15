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

### Variables and Print

The code generator handles variable assignments and the built-in `print()` function:

```grit
a = 5
b = 10
print('sum: %d', a + b)
```

Generates:

```rust
fn main() {
    let a = 5;
    let b = 10;
    println!("sum: {}", a + b);
}
```

### User-Defined Functions

Functions are generated with typed parameters and proper Rust syntax:

```grit
fn multiply(x, y) {
  x * y
}

result = multiply(3, 4)
```

Generates:

```rust
fn multiply(x: i64, y: i64) -> i64 {
    x * y
}

fn main() {
    let result = multiply(3, 4);
}
```

Key features:
- **Typed parameters**: All parameters are typed as `i64`
- **Return type**: Functions return `i64` by default
- **Implicit returns**: The last expression in a function body becomes the return value (no semicolon)
- **Proper ordering**: User-defined functions are placed before `main()`

### Multi-Statement Functions

Functions can contain multiple statements:

```grit
fn calculate(x) {
  doubled = x * 2
  doubled + 1
}
```

Generates:

```rust
fn calculate(x: i64) -> i64 {
    let doubled = x * 2;
    doubled + 1
}
```

The last expression (`doubled + 1`) becomes the return value, while earlier statements like assignments are properly terminated with semicolons.

### Control Flow

Control flow statements are transpiled to their Rust equivalents:

```grit
if x < 10 {
  print('less than 10')
} elif x == 10 {
  print('exactly 10')
} else {
  print('greater than 10')
}
```

Generates:

```rust
if x < 10 {
    println!("less than 10");
} else if x == 10 {
    println!("exactly 10");
} else {
    println!("greater than 10");
}
```

### While Loops

While loops maintain their structure:

```grit
counter = 0
while counter < 5 {
  print('count: %d', counter)
  counter = counter + 1
}
```

Generates:

```rust
let counter = 0;
while counter < 5 {
    println!("count: {}", counter);
    let counter = counter + 1;
}
```

### Comparison Operators

All comparison operators are directly mapped to Rust equivalents:
- `==` → `==`
- `!=` → `!=`
- `<` → `<`
- `<=` → `<=`
- `>` → `>`
- `>=` → `>=`
