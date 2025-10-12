use grit::lexer::{TokenType, Tokenizer};

/// Tests that directly call next_token() to ensure line 107 coverage
/// (the return statement for operator tokens)

#[test]
fn test_next_token_plus() {
    let mut tokenizer = Tokenizer::new("+");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Plus);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_minus() {
    let mut tokenizer = Tokenizer::new("-");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Minus);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_multiply() {
    let mut tokenizer = Tokenizer::new("*");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Multiply);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_divide() {
    let mut tokenizer = Tokenizer::new("/");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Divide);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_left_paren() {
    let mut tokenizer = Tokenizer::new("(");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::LeftParen);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_right_paren() {
    let mut tokenizer = Tokenizer::new(")");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::RightParen);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_integer() {
    let mut tokenizer = Tokenizer::new("42");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Integer(42));
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_eof() {
    let mut tokenizer = Tokenizer::new("");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Eof);
    assert_eq!(token.line, 1);
    assert_eq!(token.column, 1);
}

#[test]
fn test_next_token_sequence() {
    let mut tokenizer = Tokenizer::new("1 + 2");

    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Integer(1));

    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Plus);

    let token3 = tokenizer.next_token();
    assert_eq!(token3.token_type, TokenType::Integer(2));

    let token4 = tokenizer.next_token();
    assert_eq!(token4.token_type, TokenType::Eof);
}

#[test]
fn test_next_token_operators_in_sequence() {
    let mut tokenizer = Tokenizer::new("+-*/()");

    assert_eq!(tokenizer.next_token().token_type, TokenType::Plus);
    assert_eq!(tokenizer.next_token().token_type, TokenType::Minus);
    assert_eq!(tokenizer.next_token().token_type, TokenType::Multiply);
    assert_eq!(tokenizer.next_token().token_type, TokenType::Divide);
    assert_eq!(tokenizer.next_token().token_type, TokenType::LeftParen);
    assert_eq!(tokenizer.next_token().token_type, TokenType::RightParen);
    assert_eq!(tokenizer.next_token().token_type, TokenType::Eof);
}

#[test]
fn test_next_token_with_whitespace() {
    let mut tokenizer = Tokenizer::new("  +  ");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Plus);
    // Should skip leading whitespace, so column is where + is
    assert_eq!(token.column, 3);
}

#[test]
fn test_next_token_preserves_position() {
    let mut tokenizer = Tokenizer::new("+ -");

    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Plus);
    assert_eq!(token1.line, 1);
    assert_eq!(token1.column, 1);

    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Minus);
    assert_eq!(token2.line, 1);
    assert_eq!(token2.column, 3);
}
