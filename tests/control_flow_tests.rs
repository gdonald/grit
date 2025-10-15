use grit::codegen::CodeGenerator;
use grit::lexer::Tokenizer;
use grit::parser::{BinaryOperator, Expr, Parser, Program, Statement};

// Lexer tests for control flow tokens

#[test]
fn test_tokenize_if_keyword() {
    let mut tokenizer = Tokenizer::new("if");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // if, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::If);
}

#[test]
fn test_tokenize_elif_keyword() {
    let mut tokenizer = Tokenizer::new("elif");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // elif, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Elif);
}

#[test]
fn test_tokenize_else_keyword() {
    let mut tokenizer = Tokenizer::new("else");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // else, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Else);
}

#[test]
fn test_tokenize_while_keyword() {
    let mut tokenizer = Tokenizer::new("while");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // while, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::While);
}

#[test]
fn test_tokenize_equal_equal() {
    let mut tokenizer = Tokenizer::new("==");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // ==, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::EqualEqual);
}

#[test]
fn test_tokenize_not_equal() {
    let mut tokenizer = Tokenizer::new("!=");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // !=, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::NotEqual);
}

#[test]
fn test_tokenize_less_than() {
    let mut tokenizer = Tokenizer::new("<");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // <, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::LessThan);
}

#[test]
fn test_tokenize_less_than_or_equal() {
    let mut tokenizer = Tokenizer::new("<=");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // <=, EOF
    assert_eq!(
        tokens[0].token_type,
        grit::lexer::TokenType::LessThanOrEqual
    );
}

#[test]
fn test_tokenize_greater_than() {
    let mut tokenizer = Tokenizer::new(">");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // >, EOF
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::GreaterThan);
}

#[test]
fn test_tokenize_greater_than_or_equal() {
    let mut tokenizer = Tokenizer::new(">=");
    let tokens = tokenizer.tokenize();
    assert_eq!(tokens.len(), 2); // >=, EOF
    assert_eq!(
        tokens[0].token_type,
        grit::lexer::TokenType::GreaterThanOrEqual
    );
}

// Parser tests for control flow

#[test]
fn test_parse_simple_if() {
    let input = "if a < b { print('a < b') }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        } => {
            assert!(matches!(condition, Expr::BinaryOp { .. }));
            assert_eq!(then_branch.len(), 1);
            assert_eq!(elif_branches.len(), 0);
            assert!(else_branch.is_none());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parse_if_else() {
    let input = "if a < b { print('less') } else { print('not less') }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::If {
            condition: _,
            then_branch,
            elif_branches,
            else_branch,
        } => {
            assert_eq!(then_branch.len(), 1);
            assert_eq!(elif_branches.len(), 0);
            assert!(else_branch.is_some());
            assert_eq!(else_branch.as_ref().unwrap().len(), 1);
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parse_if_elif_else() {
    let input =
        "if a < b { print('less') } elif a > b { print('greater') } else { print('equal') }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::If {
            condition: _,
            then_branch,
            elif_branches,
            else_branch,
        } => {
            assert_eq!(then_branch.len(), 1);
            assert_eq!(elif_branches.len(), 1);
            assert!(else_branch.is_some());
        }
        _ => panic!("Expected if statement"),
    }
}

#[test]
fn test_parse_while_loop() {
    let input = "while x < 10 { x = x + 1 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    match &program.statements[0] {
        Statement::While { condition, body } => {
            assert!(matches!(condition, Expr::BinaryOp { .. }));
            assert_eq!(body.len(), 1);
        }
        _ => panic!("Expected while statement"),
    }
}

#[test]
fn test_parse_comparison_operators() {
    let operators = vec!["==", "!=", "<", "<=", ">", ">="];

    for op in operators {
        let input = format!("a {} b", op);
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression(Expr::BinaryOp { .. }) => {}
            _ => panic!("Expected binary operation for operator {}", op),
        }
    }
}

// Code generation tests

#[test]
fn test_generate_simple_if() {
    let program = Program {
        statements: vec![Statement::If {
            condition: Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                op: BinaryOperator::LessThan,
                right: Box::new(Expr::Identifier("b".to_string())),
            },
            then_branch: vec![Statement::Expression(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![Expr::String("a < b".to_string())],
            })],
            elif_branches: vec![],
            else_branch: None,
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("if a < b {"));
    assert!(code.contains("println!(\"a < b\");"));
}

#[test]
fn test_generate_if_else() {
    let program = Program {
        statements: vec![Statement::If {
            condition: Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                op: BinaryOperator::EqualEqual,
                right: Box::new(Expr::Identifier("b".to_string())),
            },
            then_branch: vec![Statement::Expression(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![Expr::String("equal".to_string())],
            })],
            elif_branches: vec![],
            else_branch: Some(vec![Statement::Expression(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![Expr::String("not equal".to_string())],
            })]),
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("if a == b {"));
    assert!(code.contains("} else {"));
    assert!(code.contains("println!(\"equal\");"));
    assert!(code.contains("println!(\"not equal\");"));
}

#[test]
fn test_generate_if_elif_else() {
    let program = Program {
        statements: vec![Statement::If {
            condition: Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                op: BinaryOperator::LessThan,
                right: Box::new(Expr::Identifier("b".to_string())),
            },
            then_branch: vec![Statement::Expression(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![Expr::String("less".to_string())],
            })],
            elif_branches: vec![(
                Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    op: BinaryOperator::GreaterThan,
                    right: Box::new(Expr::Identifier("b".to_string())),
                },
                vec![Statement::Expression(Expr::FunctionCall {
                    name: "print".to_string(),
                    args: vec![Expr::String("greater".to_string())],
                })],
            )],
            else_branch: Some(vec![Statement::Expression(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![Expr::String("equal".to_string())],
            })]),
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("if a < b {"));
    assert!(code.contains("} else if a > b {"));
    assert!(code.contains("} else {"));
}

#[test]
fn test_generate_while_loop() {
    let program = Program {
        statements: vec![Statement::While {
            condition: Expr::BinaryOp {
                left: Box::new(Expr::Identifier("x".to_string())),
                op: BinaryOperator::LessThan,
                right: Box::new(Expr::Integer(10)),
            },
            body: vec![Statement::Assignment {
                name: "x".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("x".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(Expr::Integer(1)),
                },
            }],
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("while x < 10 {"));
    assert!(code.contains("let x = x + 1;"));
}

#[test]
fn test_generate_all_comparison_operators() {
    let operators = vec![
        (BinaryOperator::EqualEqual, "=="),
        (BinaryOperator::NotEqual, "!="),
        (BinaryOperator::LessThan, "<"),
        (BinaryOperator::LessThanOrEqual, "<="),
        (BinaryOperator::GreaterThan, ">"),
        (BinaryOperator::GreaterThanOrEqual, ">="),
    ];

    for (op, symbol) in operators {
        let program = Program {
            statements: vec![Statement::Expression(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("a".to_string())),
                op,
                right: Box::new(Expr::Identifier("b".to_string())),
            })],
        };

        let code = CodeGenerator::generate_program(&program);
        assert!(code.contains(&format!("a {} b", symbol)));
    }
}
