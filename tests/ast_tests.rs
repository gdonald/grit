// Tests for src/parser/ast.rs
use grit::parser::{BinaryOperator, Expr, Program, Statement};

// BinaryOperator tests

#[test]
fn test_binary_operator_precedence_arithmetic() {
    assert_eq!(BinaryOperator::Add.precedence(), 1);
    assert_eq!(BinaryOperator::Subtract.precedence(), 1);
    assert_eq!(BinaryOperator::Multiply.precedence(), 2);
    assert_eq!(BinaryOperator::Divide.precedence(), 2);
}

#[test]
fn test_binary_operator_precedence_comparison() {
    assert_eq!(BinaryOperator::EqualEqual.precedence(), 0);
    assert_eq!(BinaryOperator::NotEqual.precedence(), 0);
    assert_eq!(BinaryOperator::LessThan.precedence(), 0);
    assert_eq!(BinaryOperator::LessThanOrEqual.precedence(), 0);
    assert_eq!(BinaryOperator::GreaterThan.precedence(), 0);
    assert_eq!(BinaryOperator::GreaterThanOrEqual.precedence(), 0);
}

#[test]
fn test_binary_operator_display_arithmetic() {
    assert_eq!(format!("{}", BinaryOperator::Add), "+");
    assert_eq!(format!("{}", BinaryOperator::Subtract), "-");
    assert_eq!(format!("{}", BinaryOperator::Multiply), "*");
    assert_eq!(format!("{}", BinaryOperator::Divide), "/");
}

#[test]
fn test_binary_operator_display_comparison() {
    assert_eq!(format!("{}", BinaryOperator::EqualEqual), "==");
    assert_eq!(format!("{}", BinaryOperator::NotEqual), "!=");
    assert_eq!(format!("{}", BinaryOperator::LessThan), "<");
    assert_eq!(format!("{}", BinaryOperator::LessThanOrEqual), "<=");
    assert_eq!(format!("{}", BinaryOperator::GreaterThan), ">");
    assert_eq!(format!("{}", BinaryOperator::GreaterThanOrEqual), ">=");
}

// Expr Display tests

#[test]
fn test_expr_display_integer() {
    let expr = Expr::Integer(42);
    assert_eq!(format!("{}", expr), "42");
}

#[test]
fn test_expr_display_string() {
    let expr = Expr::String("hello".to_string());
    assert_eq!(format!("{}", expr), "'hello'");
}

#[test]
fn test_expr_display_identifier() {
    let expr = Expr::Identifier("x".to_string());
    assert_eq!(format!("{}", expr), "x");
}

#[test]
fn test_expr_display_binary_op() {
    let expr = Expr::BinaryOp {
        left: Box::new(Expr::Integer(1)),
        op: BinaryOperator::Add,
        right: Box::new(Expr::Integer(2)),
    };
    assert_eq!(format!("{}", expr), "(1 + 2)");
}

#[test]
fn test_expr_display_grouped() {
    let expr = Expr::Grouped(Box::new(Expr::Integer(42)));
    assert_eq!(format!("{}", expr), "(42)");
}

#[test]
fn test_expr_display_function_call_no_args() {
    let expr = Expr::FunctionCall {
        name: "foo".to_string(),
        args: vec![],
    };
    assert_eq!(format!("{}", expr), "foo()");
}

#[test]
fn test_expr_display_function_call_with_args() {
    let expr = Expr::FunctionCall {
        name: "add".to_string(),
        args: vec![Expr::Integer(1), Expr::Integer(2), Expr::Integer(3)],
    };
    assert_eq!(format!("{}", expr), "add(1, 2, 3)");
}

#[test]
fn test_expr_display_field_access() {
    let expr = Expr::FieldAccess {
        object: Box::new(Expr::Identifier("obj".to_string())),
        field: "field".to_string(),
    };
    assert_eq!(format!("{}", expr), "obj.field");
}

#[test]
fn test_expr_display_method_call_no_args() {
    let expr = Expr::MethodCall {
        object: Box::new(Expr::Identifier("obj".to_string())),
        method: "method".to_string(),
        args: vec![],
    };
    assert_eq!(format!("{}", expr), "obj.method()");
}

#[test]
fn test_expr_display_method_call_with_args() {
    let expr = Expr::MethodCall {
        object: Box::new(Expr::Identifier("Point".to_string())),
        method: "new".to_string(),
        args: vec![Expr::Integer(3), Expr::Integer(4)],
    };
    assert_eq!(format!("{}", expr), "Point.new(3, 4)");
}

#[test]
fn test_expr_display_complex() {
    let expr = Expr::BinaryOp {
        left: Box::new(Expr::BinaryOp {
            left: Box::new(Expr::Integer(1)),
            op: BinaryOperator::Add,
            right: Box::new(Expr::Integer(2)),
        }),
        op: BinaryOperator::Multiply,
        right: Box::new(Expr::Integer(3)),
    };
    assert_eq!(format!("{}", expr), "((1 + 2) * 3)");
}

// Statement Display tests

#[test]
fn test_statement_display_function_def_no_params() {
    let stmt = Statement::FunctionDef {
        name: "main".to_string(),
        params: vec![],
        body: vec![],
    };
    assert_eq!(format!("{}", stmt), "fn main()");
}

#[test]
fn test_statement_display_function_def_with_params() {
    let stmt = Statement::FunctionDef {
        name: "add".to_string(),
        params: vec!["x".to_string(), "y".to_string()],
        body: vec![],
    };
    assert_eq!(format!("{}", stmt), "fn add(x, y)");
}

#[test]
fn test_statement_display_class_def() {
    let stmt = Statement::ClassDef {
        name: "Point".to_string(),
    };
    assert_eq!(format!("{}", stmt), "class Point");
}

#[test]
fn test_statement_display_method_def_no_params() {
    let stmt = Statement::MethodDef {
        class_name: "Foo".to_string(),
        method_name: "new".to_string(),
        params: vec![],
        body: vec![],
    };
    assert_eq!(format!("{}", stmt), "fn Foo > new()");
}

#[test]
fn test_statement_display_method_def_with_params() {
    let stmt = Statement::MethodDef {
        class_name: "Point".to_string(),
        method_name: "new".to_string(),
        params: vec!["x".to_string(), "y".to_string()],
        body: vec![],
    };
    assert_eq!(format!("{}", stmt), "fn Point > new(x, y)");
}

#[test]
fn test_statement_display_assignment() {
    let stmt = Statement::Assignment {
        name: "x".to_string(),
        value: Expr::Integer(42),
    };
    assert_eq!(format!("{}", stmt), "x = 42");
}

#[test]
fn test_statement_display_if_simple() {
    let stmt = Statement::If {
        condition: Expr::Identifier("x".to_string()),
        then_branch: vec![],
        elif_branches: vec![],
        else_branch: None,
    };
    assert_eq!(format!("{}", stmt), "if x");
}

#[test]
fn test_statement_display_if_with_elif() {
    let stmt = Statement::If {
        condition: Expr::Identifier("x".to_string()),
        then_branch: vec![],
        elif_branches: vec![(Expr::Identifier("y".to_string()), vec![])],
        else_branch: None,
    };
    assert_eq!(format!("{}", stmt), "if x + 1 elif(s)");
}

#[test]
fn test_statement_display_if_with_multiple_elif() {
    let stmt = Statement::If {
        condition: Expr::Identifier("x".to_string()),
        then_branch: vec![],
        elif_branches: vec![
            (Expr::Identifier("y".to_string()), vec![]),
            (Expr::Identifier("z".to_string()), vec![]),
        ],
        else_branch: None,
    };
    assert_eq!(format!("{}", stmt), "if x + 2 elif(s)");
}

#[test]
fn test_statement_display_if_with_else() {
    let stmt = Statement::If {
        condition: Expr::Identifier("x".to_string()),
        then_branch: vec![],
        elif_branches: vec![],
        else_branch: Some(vec![]),
    };
    assert_eq!(format!("{}", stmt), "if x + else");
}

#[test]
fn test_statement_display_if_with_elif_and_else() {
    let stmt = Statement::If {
        condition: Expr::Identifier("x".to_string()),
        then_branch: vec![],
        elif_branches: vec![(Expr::Identifier("y".to_string()), vec![])],
        else_branch: Some(vec![]),
    };
    assert_eq!(format!("{}", stmt), "if x + 1 elif(s) + else");
}

#[test]
fn test_statement_display_while() {
    let stmt = Statement::While {
        condition: Expr::BinaryOp {
            left: Box::new(Expr::Identifier("x".to_string())),
            op: BinaryOperator::LessThan,
            right: Box::new(Expr::Integer(10)),
        },
        body: vec![],
    };
    assert_eq!(format!("{}", stmt), "while (x < 10)");
}

#[test]
fn test_statement_display_expression() {
    let stmt = Statement::Expression(Expr::Integer(42));
    assert_eq!(format!("{}", stmt), "42");
}

// Program Display tests

#[test]
fn test_program_display_empty() {
    let program = Program { statements: vec![] };
    assert_eq!(format!("{}", program), "");
}

#[test]
fn test_program_display_single_statement() {
    let program = Program {
        statements: vec![Statement::Assignment {
            name: "x".to_string(),
            value: Expr::Integer(42),
        }],
    };
    assert_eq!(format!("{}", program), "x = 42");
}

#[test]
fn test_program_display_multiple_statements() {
    let program = Program {
        statements: vec![
            Statement::Assignment {
                name: "x".to_string(),
                value: Expr::Integer(1),
            },
            Statement::Assignment {
                name: "y".to_string(),
                value: Expr::Integer(2),
            },
            Statement::Expression(Expr::BinaryOp {
                left: Box::new(Expr::Identifier("x".to_string())),
                op: BinaryOperator::Add,
                right: Box::new(Expr::Identifier("y".to_string())),
            }),
        ],
    };
    assert_eq!(format!("{}", program), "x = 1\ny = 2\n(x + y)");
}

// Clone and PartialEq tests

#[test]
fn test_binary_operator_clone() {
    let op = BinaryOperator::Add;
    let cloned = op.clone();
    assert_eq!(op, cloned);
}

#[test]
fn test_expr_clone() {
    let expr = Expr::Integer(42);
    let cloned = expr.clone();
    assert_eq!(expr, cloned);
}

#[test]
fn test_statement_clone() {
    let stmt = Statement::Assignment {
        name: "x".to_string(),
        value: Expr::Integer(42),
    };
    let cloned = stmt.clone();
    assert_eq!(stmt, cloned);
}

#[test]
fn test_program_clone() {
    let program = Program {
        statements: vec![Statement::Expression(Expr::Integer(42))],
    };
    let cloned = program.clone();
    assert_eq!(program, cloned);
}

#[test]
fn test_expr_partial_eq_different() {
    let expr1 = Expr::Integer(1);
    let expr2 = Expr::Integer(2);
    assert_ne!(expr1, expr2);
}

#[test]
fn test_statement_partial_eq_different() {
    let stmt1 = Statement::Assignment {
        name: "x".to_string(),
        value: Expr::Integer(1),
    };
    let stmt2 = Statement::Assignment {
        name: "x".to_string(),
        value: Expr::Integer(2),
    };
    assert_ne!(stmt1, stmt2);
}
