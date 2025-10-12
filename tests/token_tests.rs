use grit::lexer::{Token, TokenType};

#[test]
fn test_token_creation() {
    let token = Token::new(TokenType::Integer(42), 1, 5);

    assert_eq!(token.token_type, TokenType::Integer(42));
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 5);
}

#[test]
fn test_token_equality() {
    let token1 = Token::new(TokenType::Plus, 1, 1);
    let token2 = Token::new(TokenType::Plus, 1, 1);

    assert_eq!(token1, token2);
}

#[test]
fn test_token_type_equality() {
    assert_eq!(TokenType::Plus, TokenType::Plus);
    assert_eq!(TokenType::Minus, TokenType::Minus);
    assert_eq!(TokenType::Multiply, TokenType::Multiply);
    assert_eq!(TokenType::Divide, TokenType::Divide);
    assert_eq!(TokenType::LeftParen, TokenType::LeftParen);
    assert_eq!(TokenType::RightParen, TokenType::RightParen);
    assert_eq!(TokenType::Eof, TokenType::Eof);
}

#[test]
fn test_integer_token_type_equality() {
    assert_eq!(TokenType::Integer(42), TokenType::Integer(42));
    assert_ne!(TokenType::Integer(42), TokenType::Integer(43));
}

#[test]
fn test_token_clone() {
    let token = Token::new(TokenType::Integer(100), 2, 10);
    let cloned_token = token.clone();

    assert_eq!(token, cloned_token);
    assert_eq!(cloned_token.token_type, TokenType::Integer(100));
    assert_eq!(cloned_token.line, 2);
    assert_eq!(cloned_token.column, 10);
}
