/// Abstract Syntax Tree node for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer literal
    Integer(i64),

    /// Binary operation (left operand, operator, right operand)
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },

    /// Grouped expression (parentheses)
    Grouped(Box<Expr>),
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BinaryOperator {
    /// Returns the precedence of the operator (higher = binds tighter)
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Add | BinaryOperator::Subtract => 1,
            BinaryOperator::Multiply | BinaryOperator::Divide => 2,
        }
    }
}

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Subtract => write!(f, "-"),
            BinaryOperator::Multiply => write!(f, "*"),
            BinaryOperator::Divide => write!(f, "/"),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Integer(n) => write!(f, "{}", n),
            Expr::BinaryOp { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expr::Grouped(expr) => write!(f, "({})", expr),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
