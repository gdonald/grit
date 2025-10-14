use grit::codegen::CodeGenerator;
use grit::lexer::Tokenizer;
use grit::parser::Program;
use grit::parser::{Expr, Parser, Statement};

#[test]
fn test_tokenize_fn_keyword() {
    let mut tokenizer = Tokenizer::new("fn");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2); // fn, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Fn);
}

#[test]
fn test_tokenize_braces() {
    let mut tokenizer = Tokenizer::new("{}");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 3); // {, }, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::LeftBrace);
    assert_eq!(tokens[1].token_type, grit::lexer::TokenType::RightBrace);
}

#[test]
fn test_tokenize_simple_function() {
    let mut tokenizer = Tokenizer::new("fn add(a, b) { a + b }");
    let tokens = tokenizer.tokenize();

    // fn, add, (, a, ,, b, ), {, a, +, b, }, EOF
    assert_eq!(tokens.len(), 13);
}

#[test]
fn test_parse_empty_function() {
    let mut tokenizer = Tokenizer::new("fn foo() { }");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::FunctionDef { name, params, body } => {
            assert_eq!(name, "foo");
            assert_eq!(params.len(), 0);
            assert_eq!(body.len(), 0);
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_parse_function_with_params() {
    let mut tokenizer = Tokenizer::new("fn add(a, b) { a + b }");
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::FunctionDef { name, params, body } => {
            assert_eq!(name, "add");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "a");
            assert_eq!(params[1], "b");
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_parse_function_with_multiline_body() {
    let input = "fn test(x) {\n  a = x + 1\n  a\n}";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::FunctionDef { name, params, body } => {
            assert_eq!(name, "test");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "x");
            assert_eq!(body.len(), 2);
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_parse_function_and_main_code() {
    let input = "fn add(a, b) { a + b }\nc = add(1, 2)";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 2);
    assert!(matches!(
        &program.statements[0],
        Statement::FunctionDef { .. }
    ));
    assert!(matches!(
        &program.statements[1],
        Statement::Assignment { .. }
    ));
}

#[test]
fn test_generate_empty_function() {
    let program = Program {
        statements: vec![Statement::FunctionDef {
            name: "foo".to_string(),
            params: vec![],
            body: vec![],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn foo() -> i64"));
}

#[test]
fn test_generate_function_with_one_param() {
    let program = Program {
        statements: vec![Statement::FunctionDef {
            name: "double".to_string(),
            params: vec!["x".to_string()],
            body: vec![Statement::Expression(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("x".to_string())),
                op: grit::parser::BinaryOperator::Multiply,
                right: Box::new(Expr::Integer(2)),
            })],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn double(x: i64) -> i64"));
    assert!(code.contains("x * 2"));
}

#[test]
fn test_generate_function_with_multiple_params() {
    let program = Program {
        statements: vec![Statement::FunctionDef {
            name: "add".to_string(),
            params: vec!["a".to_string(), "b".to_string()],
            body: vec![Statement::Expression(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                op: grit::parser::BinaryOperator::Add,
                right: Box::new(Expr::Identifier("b".to_string())),
            })],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn add(a: i64, b: i64) -> i64"));
    assert!(code.contains("a + b"));
}

#[test]
fn test_generate_function_with_assignment_in_body() {
    let program = Program {
        statements: vec![Statement::FunctionDef {
            name: "test".to_string(),
            params: vec!["x".to_string()],
            body: vec![
                Statement::Assignment {
                    name: "result".to_string(),
                    value: Expr::BinaryOp {
                        left: Box::new(Expr::Identifier("x".to_string())),
                        op: grit::parser::BinaryOperator::Add,
                        right: Box::new(Expr::Integer(1)),
                    },
                },
                Statement::Expression(Expr::Identifier("result".to_string())),
            ],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn test(x: i64) -> i64"));
    assert!(code.contains("let result = x + 1;"));
    assert!(code.contains("result"));
}

#[test]
fn test_generate_complete_program_with_function() {
    let program = Program {
        statements: vec![
            Statement::FunctionDef {
                name: "add".to_string(),
                params: vec!["a".to_string(), "b".to_string()],
                body: vec![Statement::Expression(Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    op: grit::parser::BinaryOperator::Add,
                    right: Box::new(Expr::Identifier("b".to_string())),
                })],
            },
            Statement::Assignment {
                name: "result".to_string(),
                value: Expr::FunctionCall {
                    name: "add".to_string(),
                    args: vec![Expr::Integer(1), Expr::Integer(2)],
                },
            },
        ],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn add(a: i64, b: i64) -> i64"));
    assert!(code.contains("fn main()"));
    assert!(code.contains("let result = add(1, 2);"));
}

#[test]
fn test_parse_function_call_to_user_function() {
    let input = "fn double(x) { x * 2 }\nresult = double(5)";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 2);
    match &program.statements[1] {
        Statement::Assignment { name, value } => {
            assert_eq!(name, "result");
            match value {
                Expr::FunctionCall { name, args } => {
                    assert_eq!(name, "double");
                    assert_eq!(args.len(), 1);
                }
                _ => panic!("Expected function call"),
            }
        }
        _ => panic!("Expected assignment"),
    }
}

#[test]
fn test_function_with_newlines_in_params() {
    let input = "fn test(\n  a,\n  b\n) {\n  a + b\n}";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::FunctionDef { name, params, body } => {
            assert_eq!(name, "test");
            assert_eq!(params.len(), 2);
            assert_eq!(params[0], "a");
            assert_eq!(params[1], "b");
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected function definition"),
    }
}

#[test]
fn test_parse_function_missing_name() {
    let input = "fn () {}";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_missing_left_paren() {
    let input = "fn foo a, b) {}";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_missing_comma_in_params() {
    let input = "fn foo(a b) {}";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_missing_left_brace() {
    let input = "fn foo() a + b }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_missing_right_brace() {
    let input = "fn foo() { a + b";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_eof_after_name() {
    use grit::lexer::Token;
    use grit::lexer::TokenType;

    let tokens = vec![
        Token::new(TokenType::Fn, 1, 1),
        Token::new(TokenType::Identifier("foo".to_string()), 1, 4),
        // Missing rest of function
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_eof_in_params() {
    use grit::lexer::Token;
    use grit::lexer::TokenType;

    let tokens = vec![
        Token::new(TokenType::Fn, 1, 1),
        Token::new(TokenType::Identifier("foo".to_string()), 1, 4),
        Token::new(TokenType::LeftParen, 1, 7),
        Token::new(TokenType::Identifier("a".to_string()), 1, 8),
        Token::new(TokenType::Comma, 1, 9),
        // Missing rest
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_parse_function_invalid_param() {
    use grit::lexer::Token;
    use grit::lexer::TokenType;

    let tokens = vec![
        Token::new(TokenType::Fn, 1, 1),
        Token::new(TokenType::Identifier("foo".to_string()), 1, 4),
        Token::new(TokenType::LeftParen, 1, 7),
        Token::new(TokenType::Integer(42), 1, 8), // Numbers can't be param names
        Token::new(TokenType::RightParen, 1, 10),
        Token::new(TokenType::LeftBrace, 1, 12),
        Token::new(TokenType::RightBrace, 1, 13),
        Token::new(TokenType::Eof, 1, 14),
    ];
    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
}

#[test]
fn test_generate_function_implicit_return() {
    // Test that the last expression in a function body becomes an implicit return
    let program = Program {
        statements: vec![Statement::FunctionDef {
            name: "get_five".to_string(),
            params: vec![],
            body: vec![Statement::Expression(Expr::Integer(5))],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn get_five() -> i64"));
    // The last expression should not have a semicolon (implicit return)
    assert!(code.contains("    5"));
    assert!(!code.contains("    5;"));
}

#[test]
fn test_generate_function_with_statement_then_expression() {
    // Test mixed statements and expression return
    let program = Program {
        statements: vec![Statement::FunctionDef {
            name: "calc".to_string(),
            params: vec!["x".to_string()],
            body: vec![
                Statement::Assignment {
                    name: "doubled".to_string(),
                    value: Expr::BinaryOp {
                        left: Box::new(Expr::Identifier("x".to_string())),
                        op: grit::parser::BinaryOperator::Multiply,
                        right: Box::new(Expr::Integer(2)),
                    },
                },
                Statement::Expression(Expr::Identifier("doubled".to_string())),
            ],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let doubled = x * 2;"));
    assert!(code.contains("    doubled\n"));
}
