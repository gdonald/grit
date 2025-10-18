use crate::parser::{BinaryOperator, Expr, Program, Statement};

/// Generates Rust source code from Grit ASTs.
pub struct CodeGenerator;

impl CodeGenerator {
    /// Generates a Rust expression string equivalent to the provided AST.
    pub fn generate_expression(ast: &Expr) -> String {
        Self::generate_expression_with_context(ast, None, false)
    }

    /// Generates a full Rust program from a Grit Program AST.
    pub fn generate_program(program: &Program) -> String {
        // Special case: if there's only one expression statement, evaluate and print it
        if program.statements.len() == 1 {
            if let Statement::Expression(expr) = &program.statements[0] {
                if !matches!(expr, Expr::FunctionCall { .. }) {
                    let expression = Self::generate_expression(expr);
                    return format!(
                        "fn main() {{\n    let result = {};\n    println!(\"{{}}\", result);\n}}\n",
                        expression
                    );
                }
            }
        }

        let mut code = String::new();
        let mut main_body = String::new();

        // Collect classes and their methods
        use std::collections::HashMap;
        let mut classes: HashMap<String, Vec<&Statement>> = HashMap::new();

        for stmt in &program.statements {
            match stmt {
                Statement::ClassDef { name } => {
                    classes.entry(name.clone()).or_default();
                }
                Statement::MethodDef { class_name, .. } => {
                    classes.entry(class_name.clone()).or_default().push(stmt);
                }
                _ => {}
            }
        }

        // Generate structs and impl blocks for each class
        for (class_name, methods) in &classes {
            // Collect all field names from all methods
            let mut fields = std::collections::HashSet::new();
            for method in methods {
                if let Statement::MethodDef { body, .. } = method {
                    Self::collect_fields(body, &mut fields);
                }
            }

            // Generate struct
            code.push_str(&format!("#[derive(Clone)]\nstruct {} {{\n", class_name));
            for field in &fields {
                code.push_str(&format!("    {}: i64,\n", field));
            }
            code.push_str("}\n\n");

            // Generate impl block
            code.push_str(&format!("impl {} {{\n", class_name));
            for method in methods {
                if let Statement::MethodDef {
                    method_name,
                    params,
                    body,
                    ..
                } = method
                {
                    code.push_str(&Self::generate_method_impl(method_name, params, body));
                }
            }
            code.push_str("}\n\n");
        }

        // Separate functions from main body statements
        for stmt in &program.statements {
            match stmt {
                Statement::FunctionDef { .. } => {
                    code.push_str(&Self::generate_statement(stmt));
                    code.push('\n');
                }
                Statement::ClassDef { .. } | Statement::MethodDef { .. } => {
                    // Already handled above
                }
                _ => {
                    main_body.push_str("    ");
                    main_body.push_str(&Self::generate_statement(stmt));
                    main_body.push('\n');
                }
            }
        }

        // Add main function
        code.push_str(&format!("fn main() {{\n{}}}\n", main_body));

        code
    }

    /// Generates Rust code for a statement.
    fn generate_statement(stmt: &Statement) -> String {
        match stmt {
            Statement::FunctionDef { name, params, body } => {
                Self::generate_function_def(name, params, body)
            }
            Statement::ClassDef { name } => {
                // Class definitions themselves don't generate code
                // They're used to track class names for struct generation
                format!("// class {}", name)
            }
            Statement::MethodDef {
                class_name,
                method_name,
                params,
                body,
            } => Self::generate_method_def(class_name, method_name, params, body),
            Statement::Assignment { name, value } => {
                format!("let {} = {};", name, Self::generate_expression(value))
            }
            Statement::If {
                condition,
                then_branch,
                elif_branches,
                else_branch,
            } => Self::generate_if_statement(condition, then_branch, elif_branches, else_branch),
            Statement::While { condition, body } => Self::generate_while_statement(condition, body),
            Statement::Expression(expr) => {
                match expr {
                    Expr::FunctionCall { name, args } if name == "print" => {
                        // Generate println! macro call from print function
                        Self::generate_print_call(args)
                    }
                    _ => {
                        format!("{};", Self::generate_expression(expr))
                    }
                }
            }
        }
    }

    /// Generates Rust code for a function definition.
    fn generate_function_def(name: &str, params: &[String], body: &[Statement]) -> String {
        let params_str = params.join(": i64, ");
        let params_with_types = if params.is_empty() {
            String::new()
        } else {
            format!("{}: i64", params_str)
        };

        let mut body_code = String::new();

        // Check if the last statement is an expression (implicit return)
        let has_implicit_return = if let Some(last) = body.last() {
            matches!(last, Statement::Expression(_))
        } else {
            false
        };

        for (i, stmt) in body.iter().enumerate() {
            body_code.push_str("    ");

            // If this is the last statement and it's an expression, make it a return
            if i == body.len() - 1 && has_implicit_return {
                if let Statement::Expression(expr) = stmt {
                    body_code.push_str(&Self::generate_expression(expr));
                } else {
                    body_code.push_str(&Self::generate_statement(stmt));
                }
            } else {
                body_code.push_str(&Self::generate_statement(stmt));
            }
            body_code.push('\n');
        }

        format!(
            "fn {}({}) -> i64 {{\n{}}}\n",
            name, params_with_types, body_code
        )
    }

    /// Generates Rust code for an if statement
    fn generate_if_statement(
        condition: &Expr,
        then_branch: &[Statement],
        elif_branches: &[(Expr, Vec<Statement>)],
        else_branch: &Option<Vec<Statement>>,
    ) -> String {
        let mut code = format!("if {} {{\n", Self::generate_expression(condition));

        // Generate then branch
        for stmt in then_branch {
            code.push_str("        ");
            code.push_str(&Self::generate_statement(stmt));
            code.push('\n');
        }

        code.push_str("    }");

        // Generate elif branches
        for (elif_condition, elif_body) in elif_branches {
            code.push_str(&format!(
                " else if {} {{\n",
                Self::generate_expression(elif_condition)
            ));

            for stmt in elif_body {
                code.push_str("        ");
                code.push_str(&Self::generate_statement(stmt));
                code.push('\n');
            }

            code.push_str("    }");
        }

        // Generate else branch
        if let Some(else_body) = else_branch {
            code.push_str(" else {\n");

            for stmt in else_body {
                code.push_str("        ");
                code.push_str(&Self::generate_statement(stmt));
                code.push('\n');
            }

            code.push_str("    }");
        }

        code
    }

    /// Generates Rust code for a while loop
    fn generate_while_statement(condition: &Expr, body: &[Statement]) -> String {
        let mut code = format!("while {} {{\n", Self::generate_expression(condition));

        // Generate body
        for stmt in body {
            code.push_str("        ");
            code.push_str(&Self::generate_statement(stmt));
            code.push('\n');
        }

        code.push_str("    }");

        code
    }

    /// Generates a println! call from print() arguments.
    fn generate_print_call(args: &[Expr]) -> String {
        if args.is_empty() {
            return "println!();".to_string();
        }

        // First argument is the format string
        let format_str = match &args[0] {
            Expr::String(s) => {
                // Convert Grit format specifiers to Rust format specifiers
                s.replace("%d", "{}").replace("%s", "{}")
            }
            _ => "{}".to_string(),
        };

        // Remaining arguments are the values
        let values: Vec<String> = args[1..].iter().map(Self::generate_expression).collect();

        if values.is_empty() {
            format!("println!(\"{}\");", format_str)
        } else {
            format!("println!(\"{}\", {});", format_str, values.join(", "))
        }
    }

    fn generate_expression_with_context(
        ast: &Expr,
        parent_precedence: Option<u8>,
        is_right_child: bool,
    ) -> String {
        match ast {
            Expr::Integer(value) => value.to_string(),
            Expr::Float(value) => value.to_string(),
            Expr::String(s) => format!("\"{}\"", s.replace("\"", "\\\"")),
            Expr::Identifier(name) => name.clone(),
            Expr::Grouped(expr) => format!(
                "({})",
                Self::generate_expression_with_context(expr, None, false)
            ),
            Expr::BinaryOp { left, op, right } => {
                let precedence = op.precedence();
                let left_str =
                    Self::generate_expression_with_context(left, Some(precedence), false);
                let right_str =
                    Self::generate_expression_with_context(right, Some(precedence), true);

                let expression = format!("{} {} {}", left_str, Self::op_symbol(op), right_str);

                let needs_parens = parent_precedence.is_some_and(|parent| {
                    precedence < parent || (precedence == parent && is_right_child)
                });

                if needs_parens {
                    format!("({})", expression)
                } else {
                    expression
                }
            }
            Expr::FunctionCall { name, args } => {
                // Handle type conversion functions
                match name.as_str() {
                    "to_int" if args.len() == 1 => {
                        let arg = Self::generate_expression_with_context(&args[0], None, false);
                        format!("({} as i64)", arg)
                    }
                    "to_float" if args.len() == 1 => {
                        let arg = Self::generate_expression_with_context(&args[0], None, false);
                        format!("({} as f64)", arg)
                    }
                    "to_string" if args.len() == 1 => {
                        let arg = Self::generate_expression_with_context(&args[0], None, false);
                        format!("{}.to_string()", arg)
                    }
                    _ => {
                        let args_str = args
                            .iter()
                            .map(|arg| Self::generate_expression_with_context(arg, None, false))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{}({})", name, args_str)
                    }
                }
            }
            Expr::FieldAccess { object, field } => {
                let object_str = Self::generate_expression_with_context(object, None, false);
                format!("{}.{}", object_str, field)
            }
            Expr::MethodCall {
                object,
                method,
                args,
            } => {
                let object_str = Self::generate_expression_with_context(object, None, false);
                let args_str = args
                    .iter()
                    .map(|arg| Self::generate_expression_with_context(arg, None, false))
                    .collect::<Vec<_>>()
                    .join(", ");

                // Check if this is a static method call (ClassName.method)
                // If object is an identifier that starts with uppercase, treat as static
                if let Expr::Identifier(class_name) = &**object {
                    if class_name.chars().next().is_some_and(|c| c.is_uppercase()) {
                        // Static method call: ClassName::method(args)
                        return format!("{}::{}({})", class_name, method, args_str);
                    }
                }

                // Instance method call: obj.method(args)
                format!("{}.{}({})", object_str, method, args_str)
            }
        }
    }

    fn op_symbol(op: &BinaryOperator) -> &'static str {
        match op {
            BinaryOperator::Add => "+",
            BinaryOperator::Subtract => "-",
            BinaryOperator::Multiply => "*",
            BinaryOperator::Divide => "/",
            BinaryOperator::EqualEqual => "==",
            BinaryOperator::NotEqual => "!=",
            BinaryOperator::LessThan => "<",
            BinaryOperator::LessThanOrEqual => "<=",
            BinaryOperator::GreaterThan => ">",
            BinaryOperator::GreaterThanOrEqual => ">=",
        }
    }

    /// Collects all field names from self.field assignments
    fn collect_fields(body: &[Statement], fields: &mut std::collections::HashSet<String>) {
        for stmt in body {
            match stmt {
                Statement::Assignment { name, .. } => {
                    // Check if this is a self.field assignment (self.field = ...)
                    if name.starts_with("self.") {
                        if let Some(field) = name.strip_prefix("self.") {
                            fields.insert(field.to_string());
                        }
                    }
                }
                Statement::If {
                    then_branch,
                    elif_branches,
                    else_branch,
                    ..
                } => {
                    Self::collect_fields(then_branch, fields);
                    for (_, branch) in elif_branches {
                        Self::collect_fields(branch, fields);
                    }
                    if let Some(else_body) = else_branch {
                        Self::collect_fields(else_body, fields);
                    }
                }
                Statement::While { body, .. } => {
                    Self::collect_fields(body, fields);
                }
                Statement::Expression(Expr::FieldAccess { object, field }) => {
                    if let Expr::Identifier(obj_name) = &**object {
                        if obj_name == "self" {
                            fields.insert(field.clone());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    /// Generates code for a method definition (not used directly, kept for compatibility)
    fn generate_method_def(
        _class_name: &str,
        _method_name: &str,
        _params: &[String],
        _body: &[Statement],
    ) -> String {
        // This is not used in the new approach but kept for compatibility
        String::new()
    }

    /// Generates code for a method implementation (inside impl block)
    fn generate_method_impl(method_name: &str, params: &[String], body: &[Statement]) -> String {
        let mut code = String::new();

        // Special handling for constructor (new method)
        if method_name == "new" {
            let params_str = params.join(": i64, ");
            let params_with_types = if params.is_empty() {
                String::new()
            } else {
                format!("{}: i64", params_str)
            };

            code.push_str(&format!(
                "    fn {}({}) -> Self {{\n",
                method_name, params_with_types
            ));

            // Collect field assignments
            let mut field_assignments = Vec::new();
            for stmt in body {
                if let Statement::Assignment { name, value } = stmt {
                    // Check if this is self.field = value
                    if name.starts_with("self.") {
                        let field = name.strip_prefix("self.").unwrap();
                        let value_str = Self::generate_expression(value);
                        field_assignments.push((field.to_string(), value_str));
                    }
                }
            }

            // Generate Self construction
            code.push_str("        Self {\n");
            for (field, value) in &field_assignments {
                code.push_str(&format!("            {}: {},\n", field, value));
            }
            code.push_str("        }\n");
            code.push_str("    }\n\n");
        } else {
            // Regular method
            let params_str = params.join(": i64, ");
            let params_with_types = if params.is_empty() {
                "&self".to_string()
            } else {
                format!("&self, {}: i64", params_str)
            };

            code.push_str(&format!(
                "    fn {}({}) -> i64 {{\n",
                method_name, params_with_types
            ));

            // Check if the last statement is an expression (implicit return)
            let has_implicit_return = if let Some(last) = body.last() {
                matches!(last, Statement::Expression(_))
            } else {
                false
            };

            for (i, stmt) in body.iter().enumerate() {
                let is_last = i == body.len() - 1;

                // Skip self.field assignments (they're handled in the constructor)
                if let Statement::Assignment { name, .. } = stmt {
                    if name.starts_with("self.") {
                        continue;
                    }
                }

                code.push_str("        ");

                // Convert field references: a -> self.a, b -> self.b
                let stmt_code = Self::generate_statement_with_self(stmt);

                if is_last && has_implicit_return {
                    // Last expression should be returned
                    if let Statement::Expression(_) = stmt {
                        code.push_str(stmt_code.trim_end_matches(';'));
                    } else {
                        code.push_str(&stmt_code);
                    }
                } else {
                    code.push_str(&stmt_code);
                }
                code.push('\n');
            }

            code.push_str("    }\n\n");
        }

        code
    }

    /// Generates a statement with self. prefix for field references
    fn generate_statement_with_self(stmt: &Statement) -> String {
        match stmt {
            Statement::Expression(expr) => {
                format!("{};", Self::generate_expression_with_self(expr))
            }
            _ => Self::generate_statement(stmt),
        }
    }

    /// Generates an expression with self. prefix for simple identifiers (field references)
    fn generate_expression_with_self(expr: &Expr) -> String {
        match expr {
            Expr::Identifier(name) if name != "self" => format!("self.{}", name),
            Expr::BinaryOp { left, op, right } => {
                let left_str = Self::generate_expression_with_self(left);
                let right_str = Self::generate_expression_with_self(right);
                format!("{} {} {}", left_str, Self::op_symbol(op), right_str)
            }
            Expr::FieldAccess { object, field } => {
                let object_str = Self::generate_expression_with_self(object);
                format!("{}.{}", object_str, field)
            }
            _ => Self::generate_expression(expr),
        }
    }
}
