# Language Tour

Grit is a scripting language that transpiles to Rust. It supports arithmetic expressions, variables, and user-defined functions. Source programs are tokenized, parsed into an Abstract Syntax Tree (AST), and then transpiled to Rust code.

## Arithmetic Expressions

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

## Variables

Grit supports variable assignments and references:

```grit
a = 1
b = 2

c = a + b

print('c: %d', c)
```

Running this produces the following Rust code:

```rust
fn main() {
    let a = 1;
    let b = 2;
    let c = a + b;
    println!("c: {}", c);
}
```

The `print()` function is a built-in that transpiles to Rust's `println!()` macro. Format specifiers like `%d` (integer) and `%s` (string) are automatically converted to Rust's `{}` placeholder syntax.

## User-Defined Functions

You can define your own functions with the `fn` keyword:

```grit
fn add(a, b) {
  a + b
}

a = 1
b = 2

c = add(a, b)
print('c: %d', c)
```

This transpiles to:

```rust
fn add(a: i64, b: i64) -> i64 {
    a + b
}

fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("c: {}", c);
}
```

Key features:
- Function parameters are automatically typed as `i64`
- Functions return `i64` by default
- The last expression in a function body is an implicit return (no semicolon)
- Functions can have multiple statements in their body
- User-defined functions are placed before `main()` in the generated code

## Control Flow

Grit supports conditional statements and loops:

```grit
a = 1
b = 2

if a < b {
  print('a < b')
} elif b < a {
  print('b < a')
} else {
  print('a == b')
}
```

This transpiles to:

```rust
fn main() {
    let a = 1;
    let b = 2;
    if a < b {
        println!("a < b");
    } else if b < a {
        println!("b < a");
    } else {
        println!("a == b");
    }
}
```

### Comparison Operators

Grit supports all standard comparison operators:
- `==` - Equal to
- `!=` - Not equal to
- `<` - Less than
- `<=` - Less than or equal to
- `>` - Greater than
- `>=` - Greater than or equal to

### While Loops

You can create loops with the `while` keyword:

```grit
x = 0
while x < 5 {
  print('x: %d', x)
  x = x + 1
}
```

This generates a standard Rust while loop:

```rust
fn main() {
    let x = 0;
    while x < 5 {
        println!("x: {}", x);
        let x = x + 1;
    }
}
```

## Next steps

- Try editing `examples/simple.grit`, `examples/variables.grit`, `examples/functions.grit`, or `examples/control-flow.grit` and rerunning the CLI
- Look at the tests in `tests/` for more usage examples
- Explore the parser implementation in `src/parser/parse.rs`
