use grit::lexer::Tokenizer;
use grit::parser::{Expr, Parser, Statement};

#[test]
fn test_parse_assignment() {
    let mut tokenizer = Tokenizer::new("a = 42");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Assignment { name, value } => {
            assert_eq!(name, "a");
            assert_eq!(*value, Expr::Integer(42));
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_multiple_assignments() {
    let mut tokenizer = Tokenizer::new("a = 1\nb = 2\nc = 3");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 3);
}

#[test]
fn test_parse_assignment_with_expression() {
    let mut tokenizer = Tokenizer::new("x = 1 + 2 * 3");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Assignment { name, .. } => {
            assert_eq!(name, "x");
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_identifier_expression() {
    let mut tokenizer = Tokenizer::new("x = a");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Assignment { name, value } => {
            assert_eq!(name, "x");
            assert_eq!(*value, Expr::Identifier("a".to_string()));
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_string_literal() {
    let mut tokenizer = Tokenizer::new("msg = 'hello'");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Assignment { name, value } => {
            assert_eq!(name, "msg");
            assert_eq!(*value, Expr::String("hello".to_string()));
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_function_call_no_args() {
    let mut tokenizer = Tokenizer::new("foo()");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression(Expr::FunctionCall { name, args }) => {
            assert_eq!(name, "foo");
            assert_eq!(args.len(), 0);
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_function_call_one_arg() {
    let mut tokenizer = Tokenizer::new("print('hello')");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression(Expr::FunctionCall { name, args }) => {
            assert_eq!(name, "print");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], Expr::String("hello".to_string()));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_function_call_multiple_args() {
    let mut tokenizer = Tokenizer::new("print('value: %d', 42)");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression(Expr::FunctionCall { name, args }) => {
            assert_eq!(name, "print");
            assert_eq!(args.len(), 2);
            assert_eq!(args[0], Expr::String("value: %d".to_string()));
            assert_eq!(args[1], Expr::Integer(42));
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_function_call_with_expression_args() {
    let mut tokenizer = Tokenizer::new("print('sum', a + b)");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Expression(Expr::FunctionCall { name, args }) => {
            assert_eq!(name, "print");
            assert_eq!(args.len(), 2);
        }
        _ => panic!("Expected function call"),
    }
}

#[test]
fn test_parse_expression_with_variables() {
    let mut tokenizer = Tokenizer::new("result = a + b * c");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::Assignment { name, .. } => {
            assert_eq!(name, "result");
        }
        _ => panic!("Expected assignment statement"),
    }
}

#[test]
fn test_parse_empty_lines() {
    let mut tokenizer = Tokenizer::new("\n\na = 1\n\n");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn test_parse_complete_program() {
    let input = "a = 1\nb = 2\nc = a + b\nprint('c: %d', c)";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 4);
}

#[test]
fn test_parse_function_call_missing_comma() {
    // Test error case: function call with missing comma between arguments
    let mut tokenizer = Tokenizer::new("func(1 2)");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_call_unexpected_eof_after_arg() {
    // Test error case: EOF after function argument without closing paren
    use grit::lexer::Token;
    use grit::lexer::TokenType;

    let tokens = vec![
        Token::new(TokenType::Identifier("func".to_string()), 1, 1),
        Token::new(TokenType::LeftParen, 1, 5),
        Token::new(TokenType::Integer(42), 1, 6),
        // Missing comma, right paren, and EOF - just ends
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_call_missing_closing_paren_after_args() {
    // Test error case: missing closing paren after parsing all arguments
    use grit::lexer::Token;
    use grit::lexer::TokenType;

    let tokens = vec![
        Token::new(TokenType::Identifier("func".to_string()), 1, 1),
        Token::new(TokenType::LeftParen, 1, 5),
        Token::new(TokenType::Integer(1), 1, 6),
        Token::new(TokenType::Comma, 1, 7),
        Token::new(TokenType::Integer(2), 1, 8),
        // No right paren, but we have EOF
        Token::new(TokenType::Eof, 1, 9),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_expression_only() {
    // Test the legacy parse_expression_only method
    let mut tokenizer = Tokenizer::new("1 + 2");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expression_only().unwrap();

    match expr {
        Expr::BinaryOp { left, right, .. } => {
            assert!(matches!(*left, Expr::Integer(1)));
            assert!(matches!(*right, Expr::Integer(2)));
        }
        _ => panic!("Expected binary operation"),
    }
}
