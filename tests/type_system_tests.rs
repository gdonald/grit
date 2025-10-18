use grit::codegen::CodeGenerator;
use grit::lexer::Tokenizer;
use grit::parser::{Expr, Parser};

#[test]
fn test_tokenize_float() {
    let mut tokenizer = Tokenizer::new("3.14");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // Float + Eof
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Float(3.14));
}

#[test]
fn test_tokenize_float_in_expression() {
    let mut tokenizer = Tokenizer::new("1.5 + 2.5");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 4); // Float + Plus + Float + Eof
}

#[test]
fn test_tokenize_integer_dot_method_call() {
    // Should NOT parse as float
    let mut tokenizer = Tokenizer::new("42.foo");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Integer(42));
    assert_eq!(tokens[1].token_type, grit::lexer::TokenType::Dot);
}

#[test]
fn test_parse_float_literal() {
    let mut tokenizer = Tokenizer::new("3.14");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        grit::parser::Statement::Expression(Expr::Float(f)) => {
            assert_eq!(*f, 3.14);
        }
        _ => panic!("Expected float expression"),
    }
}

#[test]
fn test_parse_float_arithmetic() {
    let mut tokenizer = Tokenizer::new("1.5 + 2.5");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        grit::parser::Statement::Expression(Expr::BinaryOp { left, op: _, right }) => {
            match **left {
                Expr::Float(f) => assert_eq!(f, 1.5),
                _ => panic!("Expected float left operand"),
            }
            match **right {
                Expr::Float(f) => assert_eq!(f, 2.5),
                _ => panic!("Expected float right operand"),
            }
        }
        _ => panic!("Expected binary expression"),
    }
}

#[test]
fn test_generate_float_literal() {
    let expr = Expr::Float(3.14);
    let code = CodeGenerator::generate_expression(&expr);
    assert_eq!(code, "3.14");
}

#[test]
fn test_generate_float_expression() {
    let expr = Expr::BinaryOp {
        left: Box::new(Expr::Float(1.5)),
        op: grit::parser::BinaryOperator::Add,
        right: Box::new(Expr::Float(2.5)),
    };
    let code = CodeGenerator::generate_expression(&expr);
    assert_eq!(code, "1.5 + 2.5");
}

#[test]
fn test_to_int_conversion() {
    let mut tokenizer = Tokenizer::new("to_int(3.14)");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("(3.14 as i64)"));
}

#[test]
fn test_to_float_conversion() {
    let mut tokenizer = Tokenizer::new("to_float(42)");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("(42 as f64)"));
}

#[test]
fn test_to_string_conversion() {
    let mut tokenizer = Tokenizer::new("to_string(42)");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("42.to_string()"));
}

#[test]
fn test_mixed_types_assignment() {
    let input = "x = 42\ny = 3.14\nz = 'hello'";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let x = 42;"));
    assert!(code.contains("let y = 3.14;"));
    assert!(code.contains("let z = \"hello\";"));
}

#[test]
fn test_conversion_chain() {
    let input = "x = to_string(to_int(3.14))";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("(3.14 as i64).to_string()"));
}

#[test]
fn test_float_in_arithmetic() {
    let input = "result = 10.5 * 2";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let result = 10.5 * 2;"));
}

#[test]
fn test_integer_and_float_mixed() {
    let input = "result = 5 + 2.5";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let result = 5 + 2.5;"));
}
