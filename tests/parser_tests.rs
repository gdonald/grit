use grit::lexer::Tokenizer;
use grit::parser::{BinaryOperator, Expr, Parser, Statement};

/// Helper function to parse a string as a single expression
fn parse_string(input: &str) -> Result<Expr, String> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| e.to_string())?;

    // Extract the first statement's expression
    if program.statements.is_empty() {
        return Err("No statements found".to_string());
    }

    match &program.statements[0] {
        Statement::Expression(expr) => Ok(expr.clone()),
        Statement::Assignment { value, .. } => Ok(value.clone()),
        Statement::FunctionDef { .. } => Err("Unexpected function definition".to_string()),
        Statement::ClassDef { .. } => Err("Unexpected class definition".to_string()),
        Statement::MethodDef { .. } => Err("Unexpected method definition".to_string()),
        Statement::If { .. } => Err("Unexpected if statement".to_string()),
        Statement::While { .. } => Err("Unexpected while statement".to_string()),
    }
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

// Tests moved from src/parser/parse.rs

#[test]
fn test_parse_error_display_unexpected_token() {
    use grit::lexer::{Token, TokenType};
    use grit::parser::ParseError;

    let token = Token::new(TokenType::Plus, 1, 5);
    let error = ParseError::UnexpectedToken {
        expected: "integer".to_string(),
        found: token,
    };
    let msg = format!("{}", error);
    assert!(msg.contains("Expected integer"));
    assert!(msg.contains("line 1"));
    assert!(msg.contains("column 5"));
}

#[test]
fn test_parse_error_display_unexpected_eof() {
    use grit::parser::ParseError;

    let error = ParseError::UnexpectedEof {
        expected: "expression".to_string(),
    };
    let msg = format!("{}", error);
    assert!(msg.contains("Unexpected end of file"));
    assert!(msg.contains("expected expression"));
}

#[test]
fn test_parse_error_display_invalid_expression() {
    use grit::lexer::{Token, TokenType};
    use grit::parser::ParseError;

    let token = Token::new(TokenType::RightParen, 2, 10);
    let error = ParseError::InvalidExpression {
        token: token.clone(),
    };
    let msg = format!("{}", error);
    assert!(msg.contains("Invalid expression"));
    assert!(msg.contains("line 2"));
    assert!(msg.contains("column 10"));
}

// Note: The following methods are private and tested indirectly through public parse() tests:
// - current_token()
// - advance()
// - is_at_end()
// - token_to_operator()
// - parse_primary()

#[test]
fn test_parse_integer_expression() {
    let result = parse_string("123");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Expr::Integer(123));
}

#[test]
fn test_parse_grouped_expression() {
    let result = parse_string("(42)");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Expr::Grouped(Box::new(Expr::Integer(42))));
}

#[test]
fn test_parse_empty_input() {
    use grit::parser::Parser;

    let tokens = vec![];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    // Empty input returns empty program, not an error
    assert!(result.is_ok());
    assert_eq!(result.unwrap().statements.len(), 0);
}

#[test]
fn test_parse_error_invalid_token_start() {
    let result = parse_string("+");
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_right_paren() {
    use grit::lexer::Tokenizer;
    use grit::parser::Parser;

    let mut tokenizer = Tokenizer::new("(42");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_parse_wrong_closing_paren() {
    use grit::lexer::{Token, TokenType};
    use grit::parser::{ParseError, Parser};

    let tokens = vec![
        Token::new(TokenType::LeftParen, 1, 1),
        Token::new(TokenType::Integer(42), 1, 2),
        Token::new(TokenType::Integer(43), 1, 4), // Wrong token instead of ')'
        Token::new(TokenType::Eof, 1, 5),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    match result.unwrap_err() {
        ParseError::UnexpectedToken { expected, .. } => {
            assert!(expected.contains("')'"));
        }
        _ => panic!("Expected UnexpectedToken error"),
    }
}

#[test]
fn test_parse_grouped_expression_unexpected_eof() {
    use grit::lexer::{Token, TokenType};
    use grit::parser::{ParseError, Parser};

    // Test EOF after consuming opening paren to trigger line 101
    let tokens = vec![
        Token::new(TokenType::LeftParen, 1, 1),
        Token::new(TokenType::Integer(42), 1, 2),
        // No closing paren, and no Eof token - completely empty after integer
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    match result.unwrap_err() {
        ParseError::UnexpectedEof { expected } => {
            assert!(expected.contains("')'"));
        }
        _ => panic!("Expected UnexpectedEof error"),
    }
}
