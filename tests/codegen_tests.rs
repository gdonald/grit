use grit::codegen::CodeGenerator;
use grit::parser::{BinaryOperator, Expr, Program, Statement};

#[test]
fn test_generate_assignment() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "x".to_string(),
            value: Expr::Integer(42),
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let x = 42;"));
}

#[test]
fn test_generate_multiple_assignments() {
    let program = Program {
        statements: vec![
            Statement::Assignment {
                name: "a".to_string(),
                value: Expr::Integer(1),
            },
            Statement::Assignment {
                name: "b".to_string(),
                value: Expr::Integer(2),
            },
        ],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let a = 1;"));
    assert!(code.contains("let b = 2;"));
}

#[test]
fn test_generate_assignment_with_expression() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "result".to_string(),
            value: Expr::BinaryOp {
                left: Box::new(Expr::Integer(1)),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Integer(2)),
            },
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let result = 1 + 2;"));
}

#[test]
fn test_generate_assignment_with_identifier() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "x".to_string(),
            value: Expr::Identifier("y".to_string()),
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let x = y;"));
}

#[test]
fn test_generate_print_no_args() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("println!();"));
}

#[test]
fn test_generate_print_string_only() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![Expr::String("hello".to_string())],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("println!(\"hello\");"));
}

#[test]
fn test_generate_print_with_format_d() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![Expr::String("value: %d".to_string()), Expr::Integer(42)],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("println!(\"value: {}\", 42);"));
}

#[test]
fn test_generate_print_with_format_s() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![
                Expr::String("name: %s".to_string()),
                Expr::String("Alice".to_string()),
            ],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("println!(\"name: {}\", \"Alice\");"));
}

#[test]
fn test_generate_print_with_variable() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![
                Expr::String("x: %d".to_string()),
                Expr::Identifier("x".to_string()),
            ],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("println!(\"x: {}\", x);"));
}

#[test]
fn test_generate_print_multiple_args() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![
                Expr::String("a=%d b=%d".to_string()),
                Expr::Integer(1),
                Expr::Integer(2),
            ],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("println!(\"a={} b={}\", 1, 2);"));
}

#[test]
fn test_generate_string_expression() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "msg".to_string(),
            value: Expr::String("hello world".to_string()),
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(
        code.contains("let msg = \"hello world\";") || code.contains("let msg = 'hello world';")
    );
}

#[test]
fn test_generate_string_with_quote() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "msg".to_string(),
            value: Expr::String("say \"hi\"".to_string()),
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    // Should escape quotes
    assert!(code.contains("\\\""));
}

#[test]
fn test_generate_function_call_in_expression() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "result".to_string(),
            value: Expr::FunctionCall {
                name: "foo".to_string(),
                args: vec![Expr::Integer(1), Expr::Integer(2)],
            },
        }],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("let result = foo(1, 2);"));
}

#[test]
fn test_generate_complete_program() {
    let program = Program {
        statements: vec![
            Statement::Assignment {
                name: "a".to_string(),
                value: Expr::Integer(1),
            },
            Statement::Assignment {
                name: "b".to_string(),
                value: Expr::Integer(2),
            },
            Statement::Assignment {
                name: "c".to_string(),
                value: Expr::BinaryOp {
                    left: Box::new(Expr::Identifier("a".to_string())),
                    op: BinaryOperator::Add,
                    right: Box::new(Expr::Identifier("b".to_string())),
                },
            },
            Statement::Expression(Expr::FunctionCall {
                name: "print".to_string(),
                args: vec![
                    Expr::String("c: %d".to_string()),
                    Expr::Identifier("c".to_string()),
                ],
            }),
        ],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("fn main()"));
    assert!(code.contains("let a = 1;"));
    assert!(code.contains("let b = 2;"));
    assert!(code.contains("let c = a + b;"));
    assert!(code.contains("println!(\"c: {}\", c);"));
}

#[test]
fn test_generate_non_print_function_call() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "other_func".to_string(),
            args: vec![Expr::Integer(42)],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    assert!(code.contains("other_func(42);"));
}

#[test]
fn test_generate_print_with_non_string_format() {
    // Test the edge case where print() is called with a non-string as first argument
    let program = Program {
        statements: vec![Statement::Expression(Expr::FunctionCall {
            name: "print".to_string(),
            args: vec![Expr::Integer(42)],
        })],
    };

    let code = CodeGenerator::generate_program(&program);
    // When first arg is not a string, it becomes the format, with no values to print
    assert!(code.contains("println!(\"{}\");"));
}

// Tests moved from src/codegen/mod.rs

fn assert_expression(expected: &str, expr: Expr) {
    let generated = CodeGenerator::generate_expression(&expr);
    assert_eq!(generated, expected);
}

#[test]
fn test_generate_integer_expression() {
    assert_expression("42", Expr::Integer(42));
}

#[test]
fn test_generate_addition_expression() {
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
fn test_generate_multiplication_expression() {
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
fn test_generate_expression_respects_precedence() {
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
fn test_generate_expression_respects_associativity() {
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
fn test_generate_expression_with_grouping() {
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
fn test_generate_expression_parenthesizes_lower_precedence_left_child() {
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
fn test_generate_expression_parenthesizes_lower_precedence_right_child() {
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
fn test_generate_program_wraps_single_expression() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::Integer(5))],
    };
    let rust_code = CodeGenerator::generate_program(&program);
    let expected = "fn main() {\n    let result = 5;\n    println!(\"{}\", result);\n}\n";
    assert_eq!(rust_code, expected);
}
