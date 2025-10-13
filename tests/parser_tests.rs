use grit::lexer::Tokenizer;
use grit::parser::{BinaryOperator, Expr, Parser};

/// Helper function to parse a string
fn parse_string(input: &str) -> Result<Expr, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    parser.parse().map_err(|e| e.to_string())
}

#[test]
fn test_parse_integer() {
    let result = parse_string("42").unwrap();
    assert_eq!(result, Expr::Integer(42));
}

#[test]
fn test_parse_addition() {
    let result = parse_string("1 + 2").unwrap();
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(1)),
            op: BinaryOperator::Add,
            right: Box::new(Expr::Integer(2)),
        }
    );
}

#[test]
fn test_parse_subtraction() {
    let result = parse_string("10 - 5").unwrap();
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(10)),
            op: BinaryOperator::Subtract,
            right: Box::new(Expr::Integer(5)),
        }
    );
}

#[test]
fn test_parse_multiplication() {
    let result = parse_string("3 * 4").unwrap();
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(3)),
            op: BinaryOperator::Multiply,
            right: Box::new(Expr::Integer(4)),
        }
    );
}

#[test]
fn test_parse_division() {
    let result = parse_string("20 / 4").unwrap();
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(20)),
            op: BinaryOperator::Divide,
            right: Box::new(Expr::Integer(4)),
        }
    );
}

#[test]
fn test_parse_precedence_mul_before_add() {
    let result = parse_string("1 + 2 * 3").unwrap();
    // Should parse as: 1 + (2 * 3)
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(1)),
            op: BinaryOperator::Add,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(2)),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(3)),
            }),
        }
    );
}

#[test]
fn test_parse_precedence_div_before_sub() {
    let result = parse_string("10 - 6 / 2").unwrap();
    // Should parse as: 10 - (6 / 2)
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(10)),
            op: BinaryOperator::Subtract,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(6)),
                op: BinaryOperator::Divide,
                right: Box::new(Expr::Integer(2)),
            }),
        }
    );
}

#[test]
fn test_parse_left_associativity_add() {
    let result = parse_string("1 + 2 + 3").unwrap();
    // Should parse as: (1 + 2) + 3
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Integer(2)),
            }),
            op: BinaryOperator::Add,
            right: Box::new(Expr::Integer(3)),
        }
    );
}

#[test]
fn test_parse_left_associativity_mul() {
    let result = parse_string("2 * 3 * 4").unwrap();
    // Should parse as: (2 * 3) * 4
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(2)),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(3)),
            }),
            op: BinaryOperator::Multiply,
            right: Box::new(Expr::Integer(4)),
        }
    );
}

#[test]
fn test_parse_parentheses() {
    let result = parse_string("(1 + 2)").unwrap();
    assert_eq!(
        result,
        Expr::Grouped(Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Integer(1)),
            op: BinaryOperator::Add,
            right: Box::new(Expr::Integer(2)),
        }))
    );
}

#[test]
fn test_parse_parentheses_override_precedence() {
    let result = parse_string("(1 + 2) * 3").unwrap();
    // Should parse as: (1 + 2) * 3
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Grouped(Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Integer(2)),
            }))),
            op: BinaryOperator::Multiply,
            right: Box::new(Expr::Integer(3)),
        }
    );
}

#[test]
fn test_parse_nested_parentheses() {
    let result = parse_string("((1 + 2) * 3)").unwrap();
    assert_eq!(
        result,
        Expr::Grouped(Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Grouped(Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Integer(2)),
            }))),
            op: BinaryOperator::Multiply,
            right: Box::new(Expr::Integer(3)),
        }))
    );
}

#[test]
fn test_parse_complex_expression() {
    let result = parse_string("(10 + 20) * (30 - 15) / 5").unwrap();
    // Should parse as: ((10 + 20) * (30 - 15)) / 5
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Grouped(Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(10)),
                    op: BinaryOperator::Add,
                    right: Box::new(Expr::Integer(20)),
                }))),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Grouped(Box::new(Expr::BinaryOp {
                    left: Box::new(Expr::Integer(30)),
                    op: BinaryOperator::Subtract,
                    right: Box::new(Expr::Integer(15)),
                }))),
            }),
            op: BinaryOperator::Divide,
            right: Box::new(Expr::Integer(5)),
        }
    );
}

#[test]
fn test_parse_no_whitespace() {
    let result = parse_string("1+2*3").unwrap();
    // Should parse as: 1 + (2 * 3)
    assert_eq!(
        result,
        Expr::BinaryOp {
            left: Box::new(Expr::Integer(1)),
            op: BinaryOperator::Add,
            right: Box::new(Expr::BinaryOp {
                left: Box::new(Expr::Integer(2)),
                op: BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(3)),
            }),
        }
    );
}

#[test]
fn test_parse_error_missing_closing_paren() {
    let result = parse_string("(1 + 2");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Expected ')'"));
}

#[test]
fn test_parse_error_missing_operand() {
    let result = parse_string("1 +");
    assert!(result.is_err());
}

#[test]
fn test_parse_error_unexpected_token() {
    let result = parse_string(")");
    assert!(result.is_err());
}
