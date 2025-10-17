// Tests moved from src/parser/ast.rs
use grit::parser::{BinaryOperator, Expr};

#[test]
fn test_binary_operator_precedence() {
    assert_eq!(BinaryOperator::Add.precedence(), 1);
    assert_eq!(BinaryOperator::Subtract.precedence(), 1);
    assert_eq!(BinaryOperator::Multiply.precedence(), 2);
    assert_eq!(BinaryOperator::Divide.precedence(), 2);
}

#[test]
fn test_binary_operator_display() {
    assert_eq!(format!("{}", BinaryOperator::Add), "+");
    assert_eq!(format!("{}", BinaryOperator::Subtract), "-");
    assert_eq!(format!("{}", BinaryOperator::Multiply), "*");
    assert_eq!(format!("{}", BinaryOperator::Divide), "/");
}

#[test]
fn test_expr_display_integer() {
    let expr = Expr::Integer(42);
    assert_eq!(format!("{}", expr), "42");
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
