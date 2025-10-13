use crate::parser::{BinaryOperator, Expr, Program, Statement};

/// Generates Rust source code from Grit ASTs.
pub struct CodeGenerator;

impl CodeGenerator {
    /// Generates a Rust expression string equivalent to the provided AST.
    pub fn generate_expression(ast: &Expr) -> String {
        Self::generate_expression_with_context(ast, None, false)
    }

    /// Generates a full Rust program from a Grit Program AST.
    pub fn generate_program(program: &Program) -> String {
        // Special case: if there's only one expression statement, evaluate and print it
        if program.statements.len() == 1 {
            if let Statement::Expression(expr) = &program.statements[0] {
                if !matches!(expr, Expr::FunctionCall { .. }) {
                    let expression = Self::generate_expression(expr);
                    return format!(
                        "fn main() {{\n    let result = {};\n    println!(\"{{}}\", result);\n}}\n",
                        expression
                    );
                }
            }
        }

        let mut body = String::new();

        for stmt in &program.statements {
            body.push_str("    ");
            body.push_str(&Self::generate_statement(stmt));
            body.push('\n');
        }

        format!("fn main() {{\n{}}}\n", body)
    }

    /// Generates Rust code for a statement.
    fn generate_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::Assignment { name, value } => {
                format!("let {} = {};", name, Self::generate_expression(value))
            }
            Statement::Expression(expr) => {
                match expr {
                    Expr::FunctionCall { name, args } if name == "print" => {
                        // Generate println! macro call from print function
                        Self::generate_print_call(args)
                    }
                    _ => {
                        format!("{};", Self::generate_expression(expr))
                    }
                }
            }
        }
    }

    /// Generates a println! call from print() arguments.
    fn generate_print_call(args: &[Expr]) -> String {
        if args.is_empty() {
            return "println!();".to_string();
        }

        // First argument is the format string
        let format_str = match &args[0] {
            Expr::String(s) => {
                // Convert Grit format specifiers to Rust format specifiers
                s.replace("%d", "{}").replace("%s", "{}")
            }
            _ => "{}".to_string(),
        };

        // Remaining arguments are the values
        let values: Vec<String> = args[1..].iter().map(Self::generate_expression).collect();

        if values.is_empty() {
            format!("println!(\"{}\");", format_str)
        } else {
            format!("println!(\"{}\", {});", format_str, values.join(", "))
        }
    }

    fn generate_expression_with_context(
        ast: &Expr,
        parent_precedence: Option<u8>,
        is_right_child: bool,
    ) -> String {
        match ast {
            Expr::Integer(value) => value.to_string(),
            Expr::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            Expr::Identifier(name) => name.clone(),
            Expr::Grouped(expr) => format!(
                "({})",
                Self::generate_expression_with_context(expr, None, false)
            ),
            Expr::BinaryOp { left, op, right } => {
                let precedence = op.precedence();
                let left_str =
                    Self::generate_expression_with_context(left, Some(precedence), false);
                let right_str =
                    Self::generate_expression_with_context(right, Some(precedence), true);

                let expression = format!("{} {} {}", left_str, Self::op_symbol(op), right_str);

                let needs_parens = parent_precedence.is_some_and(|parent| {
                    precedence < parent || (precedence == parent && is_right_child)
                });

                if needs_parens {
                    format!("({})", expression)
                } else {
                    expression
                }
            }
            Expr::FunctionCall { name, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| Self::generate_expression_with_context(arg, None, false))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", name, args_str)
            }
        }
    }

    fn op_symbol(op: &BinaryOperator) -> &'static str {
        match op {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{BinaryOperator, Expr};

    fn assert_expression(expected: &str, expr: Expr) {
        let generated = CodeGenerator::generate_expression(&expr);
        assert_eq!(generated, expected);
    }

    #[test]
    fn generate_integer() {
        assert_expression("42", Expr::Integer(42));
    }

    #[test]
    fn generate_addition() {
        assert_expression(
            "1 + 2",
            Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Integer(2)),
            },
        );
    }

    #[test]
    fn generate_multiplication() {
        assert_expression(
            "3 * 4",
            Expr::BinaryOp {
                left: Box::new(Expr::Integer(3)),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(4)),
            },
        );
    }

    #[test]
    fn generate_expression_respects_precedence() {
        assert_expression(
            "1 + 2 * 3",
            Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Add,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(2)),
                    op: BinaryOperator::Multiply,
                    right: Box::new(Expr::Integer(3)),
                }),
            },
        );
    }

    #[test]
    fn generate_expression_respects_associativity() {
        assert_expression(
            "1 - (2 - 3)",
            Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Subtract,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(2)),
                    op: BinaryOperator::Subtract,
                    right: Box::new(Expr::Integer(3)),
                }),
            },
        );
    }

    #[test]
    fn generate_expression_with_grouping() {
        assert_expression(
            "(1 + 2) * 3",
            Expr::BinaryOp {
                left: Box::new(Expr::Grouped(Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(1)),
                    op: BinaryOperator::Add,
                    right: Box::new(Expr::Integer(2)),
                }))),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(3)),
            },
        );
    }

    #[test]
    fn generate_expression_parenthesizes_lower_precedence_left_child() {
        assert_expression(
            "(1 + 2) * 3",
            Expr::BinaryOp {
                left: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(1)),
                    op: BinaryOperator::Add,
                    right: Box::new(Expr::Integer(2)),
                }),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(3)),
            },
        );
    }

    #[test]
    fn generate_expression_parenthesizes_lower_precedence_right_child() {
        assert_expression(
            "3 / (1 + 2)",
            Expr::BinaryOp {
                left: Box::new(Expr::Integer(3)),
                op: BinaryOperator::Divide,
                right: Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(1)),
                    op: BinaryOperator::Add,
                    right: Box::new(Expr::Integer(2)),
                }),
            },
        );
    }

    #[test]
    fn generate_program_wraps_expression() {
        let program = Program {
            statements: vec![Statement::Expression(Expr::Integer(5))],
        };
        let rust_code = CodeGenerator::generate_program(&program);
        let expected = "fn main() {\n    let result = 5;\n    println!(\"{}\", result);\n}\n";
        assert_eq!(rust_code, expected);
    }
}
