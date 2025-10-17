/// Statement in the program
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Function definition: fn name(params) { body }
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// Class definition: class Name
    ClassDef { name: String },

    /// Method definition: fn ClassName > methodName(params) { body }
    MethodDef {
        class_name: String,
        method_name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// Variable assignment: identifier = expression
    Assignment { name: String, value: Expr },

    /// If statement with optional elif and else branches
    If {
        condition: Expr,
        then_branch: Vec<Statement>,
        elif_branches: Vec<(Expr, Vec<Statement>)>,
        else_branch: Option<Vec<Statement>>,
    },

    /// While loop
    While {
        condition: Expr,
        body: Vec<Statement>,
    },

    /// Expression statement
    Expression(Expr),
}

/// Abstract Syntax Tree node for expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Integer literal
    Integer(i64),

    /// String literal
    String(String),

    /// Variable reference
    Identifier(String),

    /// Binary operation (left operand, operator, right operand)
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },

    /// Grouped expression (parentheses)
    Grouped(Box<Expr>),

    /// Function call: function_name(arg1, arg2, ...)
    FunctionCall { name: String, args: Vec<Expr> },

    /// Field access: object.field or self.field
    FieldAccess { object: Box<Expr>, field: String },

    /// Method call: object.method(args) or ClassName.method(args)
    MethodCall {
        object: Box<Expr>,
        method: String,
        args: Vec<Expr>,
    },
}

/// Program is a list of statements
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    // Comparison operators
    EqualEqual,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl BinaryOperator {
    /// Returns the precedence of the operator (higher = binds tighter)
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::EqualEqual
            | BinaryOperator::NotEqual
            | BinaryOperator::LessThan
            | BinaryOperator::LessThanOrEqual
            | BinaryOperator::GreaterThan
            | BinaryOperator::GreaterThanOrEqual => 0,
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
            BinaryOperator::EqualEqual => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessThanOrEqual => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterThanOrEqual => write!(f, ">="),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::FunctionDef {
                name,
                params,
                body: _,
            } => {
                write!(f, "fn {}({})", name, params.join(", "))
            }
            Statement::ClassDef { name } => write!(f, "class {}", name),
            Statement::MethodDef {
                class_name,
                method_name,
                params,
                body: _,
            } => {
                write!(
                    f,
                    "fn {} > {}({})",
                    class_name,
                    method_name,
                    params.join(", ")
                )
            }
            Statement::Assignment { name, value } => write!(f, "{} = {}", name, value),
            Statement::If {
                condition,
                then_branch: _,
                elif_branches,
                else_branch,
            } => {
                write!(f, "if {}", condition)?;
                if !elif_branches.is_empty() {
                    write!(f, " + {} elif(s)", elif_branches.len())?;
                }
                if else_branch.is_some() {
                    write!(f, " + else")?;
                }
                Ok(())
            }
            Statement::While { condition, body: _ } => write!(f, "while {}", condition),
            Statement::Expression(expr) => write!(f, "{}", expr),
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Integer(n) => write!(f, "{}", n),
            Expr::String(s) => write!(f, "'{}'", s),
            Expr::Identifier(id) => write!(f, "{}", id),
            Expr::BinaryOp { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expr::Grouped(expr) => write!(f, "({})", expr),
            Expr::FunctionCall { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Expr::FieldAccess { object, field } => write!(f, "{}.{}", object, field),
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                write!(f, "{}.{}(", object, method)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, stmt) in self.statements.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", stmt)?;
        }
        Ok(())
    }
}
