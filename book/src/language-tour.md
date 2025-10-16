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

## Classes

Grit supports simple object-oriented programming with classes:

```grit
class Point

fn Point > new(x, y) {
  self.x = x
  self.y = y
}

fn Point > distance {
  (x * x + y * y)
}

p = Point.new(3, 4)
```

### Class definitions

Use the `class` keyword to declare a class:

```grit
class ClassName
```

### Methods

Methods are defined using the `fn ClassName > methodName` syntax:

```grit
fn ClassName > methodName(param1, param2) {
  # method body
}
```

### Constructors

Methods named `new` are treated as constructors and should initialize instance fields:

```grit
fn Point > new(x, y) {
  self.x = x
  self.y = y
}
```

### Instance fields

Fields are created by assigning to `self.field` in the constructor. In method bodies, simple identifiers automatically reference instance fields:

```grit
fn Point > distance {
  # 'x' and 'y' automatically refer to self.x and self.y
  (x * x + y * y)
}
```

### Method calls

Methods can be called with or without parentheses (for zero-argument methods):

```grit
p = Point.new(3, 4)   # Constructor call
d = p.distance        # Method call without parentheses
d2 = p.distance()     # Method call with parentheses (same as above)
```

### Generated Rust code

Grit classes transpile to Rust structs with `impl` blocks:

```rust
#[derive(Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self {
            x: x,
            y: y,
        }
    }

    fn distance(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }
}

fn main() {
    let p = Point::new(3, 4);
}
```

## Next steps

- Try editing `examples/simple.grit`, `examples/variables.grit`, `examples/functions.grit`, `examples/control-flow.grit`, or `examples/classes.grit` and rerunning the CLI
- Look at the tests in `tests/` for more usage examples
- Explore the parser implementation in `src/parser/parse.rs`
