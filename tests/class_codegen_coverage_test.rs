// Integration test specifically designed to hit uncovered class codegen paths

use grit::codegen::CodeGenerator;
use grit::parser::{Program, Statement};

#[test]
fn test_class_with_multiple_fields() {
    // Test class with multiple methods that use different fields
    let program = Program {
        statements: vec![
            Statement::ClassDef {
                name: "Point".to_string(),
            },
            Statement::MethodDef {
                class_name: "Point".to_string(),
                method_name: "new".to_string(),
                params: vec!["x".to_string(), "y".to_string()],
                body: vec![
                    Statement::Assignment {
                        name: "self.x".to_string(),
                        value: grit::parser::Expr::Identifier("x".to_string()),
                    },
                    Statement::Assignment {
                        name: "self.y".to_string(),
                        value: grit::parser::Expr::Identifier("y".to_string()),
                    },
                ],
            },
            Statement::MethodDef {
                class_name: "Point".to_string(),
                method_name: "sum".to_string(),
                params: vec![],
                body: vec![Statement::Expression(grit::parser::Expr::BinaryOp {
                    left: Box::new(grit::parser::Expr::Identifier("x".to_string())),
                    op: grit::parser::BinaryOperator::Add,
                    right: Box::new(grit::parser::Expr::Identifier("y".to_string())),
                })],
            },
        ],
    };

    let code = CodeGenerator::generate_program(&program);

    // Verify struct generation with fields
    assert!(code.contains("#[derive(Clone)]"));
    assert!(code.contains("struct Point {"));
    assert!(code.contains("x: i64"));
    assert!(code.contains("y: i64"));

    // Verify impl block generation
    assert!(code.contains("impl Point {"));
    assert!(code.contains("fn new(x: i64, y: i64) -> Self"));
    assert!(code.contains("fn sum(&self) -> i64"));

    // Verify method bodies - constructor uses Self { x: x, y: y }
    assert!(code.contains("Self {"));
    assert!(code.contains("x: x"));
    assert!(code.contains("y: y"));
    assert!(code.contains("self.x + self.y"));
}

#[test]
fn test_class_with_no_fields() {
    // Edge case: class with method that doesn't use fields
    let program = Program {
        statements: vec![
            Statement::ClassDef {
                name: "Helper".to_string(),
            },
            Statement::MethodDef {
                class_name: "Helper".to_string(),
                method_name: "constant".to_string(),
                params: vec![],
                body: vec![Statement::Expression(grit::parser::Expr::Integer(42))],
            },
        ],
    };

    let code = CodeGenerator::generate_program(&program);

    // Should generate empty struct
    assert!(code.contains("struct Helper {"));
    assert!(code.contains("impl Helper {"));
    assert!(code.contains("fn constant(&self) -> i64"));
}

#[test]
fn test_multiple_classes() {
    // Test program with multiple classes
    let program = Program {
        statements: vec![
            Statement::ClassDef {
                name: "Foo".to_string(),
            },
            Statement::MethodDef {
                class_name: "Foo".to_string(),
                method_name: "get_a".to_string(),
                params: vec![],
                body: vec![Statement::Expression(grit::parser::Expr::Identifier(
                    "a".to_string(),
                ))],
            },
            Statement::ClassDef {
                name: "Bar".to_string(),
            },
            Statement::MethodDef {
                class_name: "Bar".to_string(),
                method_name: "get_b".to_string(),
                params: vec![],
                body: vec![Statement::Expression(grit::parser::Expr::Identifier(
                    "b".to_string(),
                ))],
            },
        ],
    };

    let code = CodeGenerator::generate_program(&program);

    // Verify both classes are generated
    assert!(code.contains("struct Foo"));
    assert!(code.contains("impl Foo"));
    assert!(code.contains("fn get_a(&self) -> i64"));

    assert!(code.contains("struct Bar"));
    assert!(code.contains("impl Bar"));
    assert!(code.contains("fn get_b(&self) -> i64"));
}
