use super::ast::{BinaryOperator, Expr, Program, Statement};
use crate::lexer::{Token, TokenType};

/// Parser for the Grit language
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

/// Parser errors
#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token },
    UnexpectedEof { expected: String },
    InvalidExpression { token: Token },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "Expected {} but found {:?} at line {}, column {}",
                    expected, found.token_type, found.line, found.column
                )
            }
            ParseError::UnexpectedEof { expected } => {
                write!(f, "Unexpected end of file, expected {}", expected)
            }
            ParseError::InvalidExpression { token } => {
                write!(
                    f,
                    "Invalid expression at line {}, column {}",
                    token.line, token.column
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

impl Parser {
    /// Creates a new parser from a vector of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    /// Returns the current token without consuming it
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Advances to the next token
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    /// Checks if the current token is EOF
    fn is_at_end(&self) -> bool {
        matches!(
            self.current_token(),
            Some(token) if token.token_type == TokenType::Eof
        ) || self.current_token().is_none()
    }

    /// Skips newline tokens
    fn skip_newlines(&mut self) {
        while let Some(token) = self.current_token() {
            if token.token_type == TokenType::Newline {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Parses the tokens into a program
    pub fn parse(&mut self) -> ParseResult<Program> {
        let mut statements = Vec::new();

        self.skip_newlines();

        while !self.is_at_end() {
            let stmt = self.parse_statement()?;
            statements.push(stmt);
            self.skip_newlines();
        }

        Ok(Program { statements })
    }

    /// Parses a single statement
    fn parse_statement(&mut self) -> ParseResult<Statement> {
        // Check if this is a class definition
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Class {
                return self.parse_class_def();
            }
        }
        // Check if this is a function definition
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Fn {
                return self.parse_function_or_method_def();
            }
            // Check if this is an if statement
            if token.token_type == TokenType::If {
                return self.parse_if_statement();
            }
            // Check if this is a while loop
            if token.token_type == TokenType::While {
                return self.parse_while_statement();
            }
        }

        // Check if this is an assignment (identifier = expression or self.field = expression)
        if let Some(token) = self.current_token() {
            // Handle simple identifier assignment
            if let TokenType::Identifier(name) = &token.token_type {
                let name = name.clone();
                // Look ahead to see if there's an equals sign
                if self.position + 1 < self.tokens.len() {
                    if let Some(next_token) = self.tokens.get(self.position + 1) {
                        if next_token.token_type == TokenType::Equals {
                            // This is an assignment
                            self.advance(); // consume identifier
                            self.advance(); // consume '='
                            let value = self.parse_expression(0)?;

                            // Consume optional newline or require EOF
                            if let Some(token) = self.current_token() {
                                if token.token_type == TokenType::Newline {
                                    self.advance();
                                }
                            }

                            return Ok(Statement::Assignment { name, value });
                        }
                    }
                }
            }

            // Handle self.field assignment
            if token.token_type == TokenType::Self_ {
                // Check if we have self.field = value
                if self.position + 1 < self.tokens.len() {
                    if let Some(dot_token) = self.tokens.get(self.position + 1) {
                        if dot_token.token_type == TokenType::Dot {
                            if let Some(field_token) = self.tokens.get(self.position + 2) {
                                if let TokenType::Identifier(field) = &field_token.token_type {
                                    let field = field.clone();
                                    if let Some(equals_token) = self.tokens.get(self.position + 3) {
                                        if equals_token.token_type == TokenType::Equals {
                                            // This is a self.field assignment
                                            self.advance(); // consume 'self'
                                            self.advance(); // consume '.'
                                            self.advance(); // consume field name
                                            self.advance(); // consume '='
                                            let value = self.parse_expression(0)?;

                                            // Consume optional newline
                                            if let Some(token) = self.current_token() {
                                                if token.token_type == TokenType::Newline {
                                                    self.advance();
                                                }
                                            }

                                            return Ok(Statement::Assignment {
                                                name: format!("self.{}", field),
                                                value,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Otherwise, parse as expression statement
        let expr = self.parse_expression(0)?;

        // Consume optional newline
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Newline {
                self.advance();
            }
        }

        Ok(Statement::Expression(expr))
    }

    /// Parses a class definition: class Name
    fn parse_class_def(&mut self) -> ParseResult<Statement> {
        // Consume 'class' keyword
        self.advance();

        // Parse class name
        let name = if let Some(token) = self.current_token() {
            if let TokenType::Identifier(name) = &token.token_type {
                let name = name.clone();
                self.advance();
                name
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "class name".to_string(),
                    found: token.clone(),
                });
            }
        } else {
            return Err(ParseError::UnexpectedEof {
                expected: "class name".to_string(),
            });
        };

        // Consume optional newline after class definition
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Newline {
                self.advance();
            }
        }

        Ok(Statement::ClassDef { name })
    }

    /// Parses a function or method definition
    /// fn name(params) { body } or fn ClassName > methodName(params) { body }
    fn parse_function_or_method_def(&mut self) -> ParseResult<Statement> {
        // Consume 'fn' keyword
        self.advance();

        // Parse first identifier (function name or class name)
        let first_name = if let Some(token) = self.current_token() {
            if let TokenType::Identifier(name) = &token.token_type {
                let name = name.clone();
                self.advance();
                name
            } else {
                return Err(ParseError::UnexpectedToken {
                    expected: "function or class name".to_string(),
                    found: token.clone(),
                });
            }
        } else {
            return Err(ParseError::UnexpectedEof {
                expected: "function or class name".to_string(),
            });
        };

        // Check if this is a method definition (look for '>')
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::GreaterThan {
                // This is a method definition (using > as arrow)
                self.advance(); // consume '>'

                // Parse method name
                let method_name = if let Some(token) = self.current_token() {
                    if let TokenType::Identifier(name) = &token.token_type {
                        let name = name.clone();
                        self.advance();
                        name
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            expected: "method name".to_string(),
                            found: token.clone(),
                        });
                    }
                } else {
                    return Err(ParseError::UnexpectedEof {
                        expected: "method name".to_string(),
                    });
                };

                let class_name = first_name;
                let (params, body) = self.parse_function_params_and_body()?;

                return Ok(Statement::MethodDef {
                    class_name,
                    method_name,
                    params,
                    body,
                });
            }
        }

        // This is a regular function definition
        let name = first_name;
        let (params, body) = self.parse_function_params_and_body()?;

        Ok(Statement::FunctionDef { name, params, body })
    }

    /// Parses function parameters and body (shared by functions and methods)
    fn parse_function_params_and_body(&mut self) -> ParseResult<(Vec<String>, Vec<Statement>)> {
        // Check if there's a '(' - if not, skip parameter parsing
        if let Some(token) = self.current_token() {
            if token.token_type != TokenType::LeftParen {
                // No parameters, skip to body parsing
                self.skip_newlines();
                let body = self.parse_function_body()?;
                return Ok((Vec::new(), body));
            }
        }

        // Expect '(' (we know it's there from the check above)
        if let Some(token) = self.current_token() {
            if token.token_type != TokenType::LeftParen {
                return Err(ParseError::UnexpectedToken {
                    expected: "'('".to_string(),
                    found: token.clone(),
                });
            }
            self.advance();
        } else {
            return Err(ParseError::UnexpectedEof {
                expected: "'('".to_string(),
            });
        }

        // Parse parameters
        let mut params = Vec::new();
        loop {
            // Skip newlines
            self.skip_newlines();

            if let Some(token) = self.current_token() {
                if token.token_type == TokenType::RightParen {
                    self.advance();
                    break;
                }

                if let TokenType::Identifier(param) = &token.token_type {
                    params.push(param.clone());
                    self.advance();

                    // Check for comma or right paren
                    self.skip_newlines();
                    if let Some(token) = self.current_token() {
                        if token.token_type == TokenType::Comma {
                            self.advance();
                        } else if token.token_type == TokenType::RightParen {
                            self.advance();
                            break;
                        } else {
                            return Err(ParseError::UnexpectedToken {
                                expected: "',' or ')'".to_string(),
                                found: token.clone(),
                            });
                        }
                    } else {
                        return Err(ParseError::UnexpectedEof {
                            expected: "',' or ')'".to_string(),
                        });
                    }
                } else {
                    return Err(ParseError::UnexpectedToken {
                        expected: "parameter name".to_string(),
                        found: token.clone(),
                    });
                }
            } else {
                return Err(ParseError::UnexpectedEof {
                    expected: "')' or parameter name".to_string(),
                });
            }
        }

        // Skip newlines before '{'
        self.skip_newlines();

        let body = self.parse_function_body()?;

        Ok((params, body))
    }

    /// Parses a function body (the statements between { and })
    fn parse_function_body(&mut self) -> ParseResult<Vec<Statement>> {
        // Expect '{'
        if let Some(token) = self.current_token() {
            if token.token_type != TokenType::LeftBrace {
                return Err(ParseError::UnexpectedToken {
                    expected: "'{'".to_string(),
                    found: token.clone(),
                });
            }
            self.advance();
        } else {
            return Err(ParseError::UnexpectedEof {
                expected: "'{'".to_string(),
            });
        }

        // Parse function body
        let mut body = Vec::new();
        self.skip_newlines();

        loop {
            if let Some(token) = self.current_token() {
                if token.token_type == TokenType::RightBrace {
                    self.advance();
                    break;
                }

                let stmt = self.parse_statement()?;
                body.push(stmt);
                self.skip_newlines();
            } else {
                return Err(ParseError::UnexpectedEof {
                    expected: "'}'".to_string(),
                });
            }
        }

        // Consume optional newline after function
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Newline {
                self.advance();
            }
        }

        Ok(body)
    }

    /// Parses an if statement with optional elif and else branches
    fn parse_if_statement(&mut self) -> ParseResult<Statement> {
        // Consume 'if'
        self.advance();

        // Parse condition
        let condition = self.parse_expression(0)?;

        // Skip newlines before '{'
        self.skip_newlines();

        // Expect '{'
        if let Some(token) = self.current_token() {
            if token.token_type != TokenType::LeftBrace {
                return Err(ParseError::UnexpectedToken {
                    expected: "'{'".to_string(),
                    found: token.clone(),
                });
            }
            self.advance();
        } else {
            return Err(ParseError::UnexpectedEof {
                expected: "'{'".to_string(),
            });
        }

        // Parse then branch
        let mut then_branch = Vec::new();
        self.skip_newlines();

        loop {
            if let Some(token) = self.current_token() {
                if token.token_type == TokenType::RightBrace {
                    self.advance();
                    break;
                }

                let stmt = self.parse_statement()?;
                then_branch.push(stmt);
                self.skip_newlines();
            } else {
                return Err(ParseError::UnexpectedEof {
                    expected: "'}'".to_string(),
                });
            }
        }

        // Parse optional elif branches
        let mut elif_branches = Vec::new();
        self.skip_newlines();

        while let Some(token) = self.current_token() {
            if token.token_type == TokenType::Elif {
                self.advance();

                // Parse elif condition
                let elif_condition = self.parse_expression(0)?;

                // Skip newlines before '{'
                self.skip_newlines();

                // Expect '{'
                if let Some(token) = self.current_token() {
                    if token.token_type != TokenType::LeftBrace {
                        return Err(ParseError::UnexpectedToken {
                            expected: "'{'".to_string(),
                            found: token.clone(),
                        });
                    }
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedEof {
                        expected: "'{'".to_string(),
                    });
                }

                // Parse elif body
                let mut elif_body = Vec::new();
                self.skip_newlines();

                loop {
                    if let Some(token) = self.current_token() {
                        if token.token_type == TokenType::RightBrace {
                            self.advance();
                            break;
                        }

                        let stmt = self.parse_statement()?;
                        elif_body.push(stmt);
                        self.skip_newlines();
                    } else {
                        return Err(ParseError::UnexpectedEof {
                            expected: "'}'".to_string(),
                        });
                    }
                }

                elif_branches.push((elif_condition, elif_body));
                self.skip_newlines();
            } else {
                break;
            }
        }

        // Parse optional else branch
        let else_branch = if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Else {
                self.advance();

                // Skip newlines before '{'
                self.skip_newlines();

                // Expect '{'
                if let Some(token) = self.current_token() {
                    if token.token_type != TokenType::LeftBrace {
                        return Err(ParseError::UnexpectedToken {
                            expected: "'{'".to_string(),
                            found: token.clone(),
                        });
                    }
                    self.advance();
                } else {
                    return Err(ParseError::UnexpectedEof {
                        expected: "'{'".to_string(),
                    });
                }

                // Parse else body
                let mut else_body = Vec::new();
                self.skip_newlines();

                loop {
                    if let Some(token) = self.current_token() {
                        if token.token_type == TokenType::RightBrace {
                            self.advance();
                            break;
                        }

                        let stmt = self.parse_statement()?;
                        else_body.push(stmt);
                        self.skip_newlines();
                    } else {
                        return Err(ParseError::UnexpectedEof {
                            expected: "'}'".to_string(),
                        });
                    }
                }

                Some(else_body)
            } else {
                None
            }
        } else {
            None
        };

        // Consume optional newline after if statement
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Newline {
                self.advance();
            }
        }

        Ok(Statement::If {
            condition,
            then_branch,
            elif_branches,
            else_branch,
        })
    }

    /// Parses a while loop
    fn parse_while_statement(&mut self) -> ParseResult<Statement> {
        // Consume 'while'
        self.advance();

        // Parse condition
        let condition = self.parse_expression(0)?;

        // Skip newlines before '{'
        self.skip_newlines();

        // Expect '{'
        if let Some(token) = self.current_token() {
            if token.token_type != TokenType::LeftBrace {
                return Err(ParseError::UnexpectedToken {
                    expected: "'{'".to_string(),
                    found: token.clone(),
                });
            }
            self.advance();
        } else {
            return Err(ParseError::UnexpectedEof {
                expected: "'{'".to_string(),
            });
        }

        // Parse body
        let mut body = Vec::new();
        self.skip_newlines();

        loop {
            if let Some(token) = self.current_token() {
                if token.token_type == TokenType::RightBrace {
                    self.advance();
                    break;
                }

                let stmt = self.parse_statement()?;
                body.push(stmt);
                self.skip_newlines();
            } else {
                return Err(ParseError::UnexpectedEof {
                    expected: "'}'".to_string(),
                });
            }
        }

        // Consume optional newline after while statement
        if let Some(token) = self.current_token() {
            if token.token_type == TokenType::Newline {
                self.advance();
            }
        }

        Ok(Statement::While { condition, body })
    }

    /// Legacy method for parsing a single expression (for backwards compatibility)
    pub fn parse_expression_only(&mut self) -> ParseResult<Expr> {
        self.parse_expression(0)
    }

    /// Parses a primary expression (integer, string, identifier, function call, or grouped expression)
    fn parse_primary(&mut self) -> ParseResult<Expr> {
        let token = self
            .current_token()
            .ok_or_else(|| ParseError::UnexpectedEof {
                expected: "expression".to_string(),
            })?;

        match &token.token_type {
            TokenType::Integer(n) => {
                let value = *n;
                self.advance();
                Ok(Expr::Integer(value))
            }
            TokenType::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Expr::String(value))
            }
            TokenType::Self_ => {
                self.advance();
                Ok(Expr::Identifier("self".to_string()))
            }
            TokenType::Identifier(name) => {
                let name = name.clone();
                self.advance();

                // Check if this is a function call
                if let Some(token) = self.current_token() {
                    if token.token_type == TokenType::LeftParen {
                        self.advance(); // consume '('

                        let mut args = Vec::new();

                        // Parse arguments
                        if let Some(token) = self.current_token() {
                            if token.token_type != TokenType::RightParen {
                                loop {
                                    args.push(self.parse_expression(0)?);

                                    if let Some(token) = self.current_token() {
                                        if token.token_type == TokenType::Comma {
                                            self.advance(); // consume ','
                                            continue;
                                        } else if token.token_type == TokenType::RightParen {
                                            break;
                                        } else {
                                            return Err(ParseError::UnexpectedToken {
                                                expected: "',' or ')'".to_string(),
                                                found: token.clone(),
                                            });
                                        }
                                    } else {
                                        return Err(ParseError::UnexpectedEof {
                                            expected: "')'".to_string(),
                                        });
                                    }
                                }
                            }
                        }

                        let token =
                            self.current_token()
                                .ok_or_else(|| ParseError::UnexpectedEof {
                                    expected: "')'".to_string(),
                                })?;

                        if token.token_type != TokenType::RightParen {
                            return Err(ParseError::UnexpectedToken {
                                expected: "')'".to_string(),
                                found: token.clone(),
                            });
                        }

                        self.advance(); // consume ')'
                        return Ok(Expr::FunctionCall { name, args });
                    }
                }

                // Otherwise, it's just an identifier
                Ok(Expr::Identifier(name))
            }
            TokenType::LeftParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression(0)?;

                let token = self
                    .current_token()
                    .ok_or_else(|| ParseError::UnexpectedEof {
                        expected: "')'".to_string(),
                    })?;

                if token.token_type != TokenType::RightParen {
                    return Err(ParseError::UnexpectedToken {
                        expected: "')'".to_string(),
                        found: token.clone(),
                    });
                }

                self.advance(); // consume ')'
                Ok(Expr::Grouped(Box::new(expr)))
            }
            _ => Err(ParseError::InvalidExpression {
                token: token.clone(),
            }),
        }
    }

    /// Converts a token type to a binary operator
    fn token_to_operator(token_type: &TokenType) -> Option<BinaryOperator> {
        match token_type {
            TokenType::Plus => Some(BinaryOperator::Add),
            TokenType::Minus => Some(BinaryOperator::Subtract),
            TokenType::Multiply => Some(BinaryOperator::Multiply),
            TokenType::Divide => Some(BinaryOperator::Divide),
            TokenType::EqualEqual => Some(BinaryOperator::EqualEqual),
            TokenType::NotEqual => Some(BinaryOperator::NotEqual),
            TokenType::LessThan => Some(BinaryOperator::LessThan),
            TokenType::LessThanOrEqual => Some(BinaryOperator::LessThanOrEqual),
            TokenType::GreaterThan => Some(BinaryOperator::GreaterThan),
            TokenType::GreaterThanOrEqual => Some(BinaryOperator::GreaterThanOrEqual),
            _ => None,
        }
    }

    /// Parses an expression using precedence climbing
    fn parse_expression(&mut self, min_precedence: u8) -> ParseResult<Expr> {
        let mut left = self.parse_primary()?;

        while let Some(token) = self.current_token() {
            if self.is_at_end() {
                break;
            }

            // Stop at statement terminators
            if matches!(
                token.token_type,
                TokenType::Newline | TokenType::Comma | TokenType::RightParen
            ) {
                break;
            }

            // Handle dot operator for field access and method calls (highest precedence)
            if token.token_type == TokenType::Dot {
                self.advance(); // consume '.'

                // Parse the field or method name
                let field = if let Some(token) = self.current_token() {
                    if let TokenType::Identifier(name) = &token.token_type {
                        let name = name.clone();
                        self.advance();
                        name
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            expected: "field or method name".to_string(),
                            found: token.clone(),
                        });
                    }
                } else {
                    return Err(ParseError::UnexpectedEof {
                        expected: "field or method name".to_string(),
                    });
                };

                // Check if this is a method call (has parentheses)
                let mut args = Vec::new();
                if let Some(token) = self.current_token() {
                    if token.token_type == TokenType::LeftParen {
                        self.advance(); // consume '('

                        // Parse arguments
                        if let Some(token) = self.current_token() {
                            if token.token_type != TokenType::RightParen {
                                loop {
                                    args.push(self.parse_expression(0)?);

                                    if let Some(token) = self.current_token() {
                                        if token.token_type == TokenType::Comma {
                                            self.advance(); // consume ','
                                            continue;
                                        } else if token.token_type == TokenType::RightParen {
                                            break;
                                        } else {
                                            return Err(ParseError::UnexpectedToken {
                                                expected: "',' or ')'".to_string(),
                                                found: token.clone(),
                                            });
                                        }
                                    } else {
                                        return Err(ParseError::UnexpectedEof {
                                            expected: "')'".to_string(),
                                        });
                                    }
                                }
                            }
                        }

                        let token =
                            self.current_token()
                                .ok_or_else(|| ParseError::UnexpectedEof {
                                    expected: "')'".to_string(),
                                })?;

                        if token.token_type != TokenType::RightParen {
                            return Err(ParseError::UnexpectedToken {
                                expected: "')'".to_string(),
                                found: token.clone(),
                            });
                        }

                        self.advance(); // consume ')'
                    }
                }

                // In Grit, obj.method is always a method call (with or without parens)
                left = Expr::MethodCall {
                    object: Box::new(left),
                    method: field,
                    args,
                };
                continue;
            }

            let op = match Self::token_to_operator(&token.token_type) {
                Some(op) => op,
                None => break,
            };

            let precedence = op.precedence();
            if precedence < min_precedence {
                break;
            }

            self.advance(); // consume operator

            let right = self.parse_expression(precedence + 1)?;

            left = Expr::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{Token, TokenType, Tokenizer};

    #[test]
    fn test_parse_error_display_unexpected_token() {
        let token = Token::new(TokenType::Plus, 1, 5);
        let error = ParseError::UnexpectedToken {
            expected: "integer".to_string(),
            found: token,
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Expected integer"));
        assert!(msg.contains("line 1"));
        assert!(msg.contains("column 5"));
    }

    #[test]
    fn test_parse_error_display_unexpected_eof() {
        let error = ParseError::UnexpectedEof {
            expected: "expression".to_string(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Unexpected end of file"));
        assert!(msg.contains("expected expression"));
    }

    #[test]
    fn test_parse_error_display_invalid_expression() {
        let token = Token::new(TokenType::RightParen, 2, 10);
        let error = ParseError::InvalidExpression {
            token: token.clone(),
        };
        let msg = format!("{}", error);
        assert!(msg.contains("Invalid expression"));
        assert!(msg.contains("line 2"));
        assert!(msg.contains("column 10"));
    }

    #[test]
    fn test_parser_current_token() {
        let tokens = vec![
            Token::new(TokenType::Integer(42), 1, 1),
            Token::new(TokenType::Eof, 1, 3),
        ];
        let parser = Parser::new(tokens);
        assert_eq!(
            parser.current_token().unwrap().token_type,
            TokenType::Integer(42)
        );
    }

    #[test]
    fn test_parser_advance() {
        let tokens = vec![
            Token::new(TokenType::Integer(1), 1, 1),
            Token::new(TokenType::Plus, 1, 3),
            Token::new(TokenType::Eof, 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        assert_eq!(
            parser.current_token().unwrap().token_type,
            TokenType::Integer(1)
        );
        parser.advance();
        assert_eq!(parser.current_token().unwrap().token_type, TokenType::Plus);
        parser.advance();
        assert_eq!(parser.current_token().unwrap().token_type, TokenType::Eof);
    }

    #[test]
    fn test_parser_is_at_end() {
        let tokens = vec![Token::new(TokenType::Eof, 1, 1)];
        let parser = Parser::new(tokens);
        assert!(parser.is_at_end());
    }

    #[test]
    fn test_token_to_operator() {
        assert_eq!(
            Parser::token_to_operator(&TokenType::Plus),
            Some(BinaryOperator::Add)
        );
        assert_eq!(
            Parser::token_to_operator(&TokenType::Minus),
            Some(BinaryOperator::Subtract)
        );
        assert_eq!(
            Parser::token_to_operator(&TokenType::Multiply),
            Some(BinaryOperator::Multiply)
        );
        assert_eq!(
            Parser::token_to_operator(&TokenType::Divide),
            Some(BinaryOperator::Divide)
        );
        assert_eq!(Parser::token_to_operator(&TokenType::LeftParen), None);
        assert_eq!(Parser::token_to_operator(&TokenType::Integer(42)), None);
    }

    #[test]
    fn test_parse_primary_integer() {
        let tokens = vec![
            Token::new(TokenType::Integer(123), 1, 1),
            Token::new(TokenType::Eof, 1, 4),
        ];
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_primary().unwrap();
        assert_eq!(expr, Expr::Integer(123));
    }

    #[test]
    fn test_parse_primary_grouped() {
        let mut tokenizer = Tokenizer::new("(42)");
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse_primary().unwrap();
        assert_eq!(expr, Expr::Grouped(Box::new(Expr::Integer(42))));
    }

    #[test]
    fn test_parse_primary_error_eof() {
        let tokens = vec![];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_primary();
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedEof { .. } => {}
            _ => panic!("Expected UnexpectedEof error"),
        }
    }

    #[test]
    fn test_parse_primary_error_invalid() {
        let tokens = vec![Token::new(TokenType::Plus, 1, 1)];
        let mut parser = Parser::new(tokens);
        let result = parser.parse_primary();
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::InvalidExpression { .. } => {}
            _ => panic!("Expected InvalidExpression error"),
        }
    }

    #[test]
    fn test_parse_missing_right_paren() {
        let mut tokenizer = Tokenizer::new("(42");
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_wrong_closing_paren() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1, 1),
            Token::new(TokenType::Integer(42), 1, 2),
            Token::new(TokenType::Integer(43), 1, 4), // Wrong token instead of ')'
            Token::new(TokenType::Eof, 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, .. } => {
                assert!(expected.contains("')'"));
            }
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_parse_grouped_expression_unexpected_eof() {
        // Test EOF after consuming opening paren to trigger line 101
        let tokens = vec![
            Token::new(TokenType::LeftParen, 1, 1),
            Token::new(TokenType::Integer(42), 1, 2),
            // No closing paren, and no Eof token - completely empty after integer
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedEof { expected } => {
                assert!(expected.contains("')'"));
            }
            _ => panic!("Expected UnexpectedEof error"),
        }
    }
}
