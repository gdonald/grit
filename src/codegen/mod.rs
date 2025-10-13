use crate::parser::{BinaryOperator, Expr};

/// Generates Rust source code from Grit ASTs.
pub struct CodeGenerator;

impl CodeGenerator {
    /// Generates a Rust expression string equivalent to the provided AST.
    pub fn generate_expression(ast: &Expr) -> String {
        Self::generate_expression_with_context(ast, None, false)
    }

    /// Generates a full Rust program that evaluates the provided AST.
    pub fn generate_program(ast: &Expr) -> String {
        let expression = Self::generate_expression(ast);

        format!(
            "fn main() {{\n    let result = {};\n    println!(\"{{}}\", result);\n}}\n",
            expression
        )
    }

    fn generate_expression_with_context(
        ast: &Expr,
        parent_precedence: Option<u8>,
        is_right_child: bool,
    ) -> String {
        match ast {
            Expr::Integer(value) => value.to_string(),
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

                let needs_parens = parent_precedence.map_or(false, |parent| {
                    precedence < parent || (precedence == parent && is_right_child)
                });

                if needs_parens {
                    format!("({})", expression)
                } else {
                    expression
                }
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
        let expr = Expr::Integer(5);
        let program = CodeGenerator::generate_program(&expr);
        let expected = "fn main() {\n    let result = 5;\n    println!(\"{}\", result);\n}\n";
        assert_eq!(program, expected);
    }
}
