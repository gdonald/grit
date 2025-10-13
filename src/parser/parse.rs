use super::ast::{BinaryOperator, Expr};
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

    /// Parses the tokens into an expression
    pub fn parse(&mut self) -> ParseResult<Expr> {
        self.parse_expression(0)
    }

    /// Parses a primary expression (integer or grouped expression)
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
        assert_eq!(parser.current_token().unwrap().token_type, TokenType::Integer(42));
    }

    #[test]
    fn test_parser_advance() {
        let tokens = vec![
            Token::new(TokenType::Integer(1), 1, 1),
            Token::new(TokenType::Plus, 1, 3),
            Token::new(TokenType::Eof, 1, 5),
        ];
        let mut parser = Parser::new(tokens);
        assert_eq!(parser.current_token().unwrap().token_type, TokenType::Integer(1));
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
            Token::new(TokenType::Integer(43), 1, 4),  // Wrong token instead of ')'
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
