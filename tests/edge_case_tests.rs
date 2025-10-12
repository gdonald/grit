use grit::lexer::{TokenType, Tokenizer};

/// Test that exercises the None branch in advance() by calling next_token repeatedly
/// after EOF is reached
#[test]
fn test_multiple_next_token_after_eof() {
    let mut tokenizer = Tokenizer::new("42");

    // First call gets the integer
    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Integer(42));

    // Second call gets EOF
    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Eof);

    // Third call still gets EOF (this exercises the None branch in advance)
    let token3 = tokenizer.next_token();
    assert_eq!(token3.token_type, TokenType::Eof);

    // Fourth call for good measure
    let token4 = tokenizer.next_token();
    assert_eq!(token4.token_type, TokenType::Eof);
}

/// Test single character followed by attempting to read beyond
#[test]
fn test_single_char_then_multiple_eofs() {
    let mut tokenizer = Tokenizer::new("+");

    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Plus);

    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Eof);

    let token3 = tokenizer.next_token();
    assert_eq!(token3.token_type, TokenType::Eof);
}

/// Test reading operators to ensure line 106 is covered
#[test]
fn test_all_operators_individually() {
    // Test Plus operator
    let mut tokenizer = Tokenizer::new("+");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Plus);

    // Test Minus operator
    let mut tokenizer = Tokenizer::new("-");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Minus);

    // Test Multiply operator
    let mut tokenizer = Tokenizer::new("*");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Multiply);

    // Test Divide operator
    let mut tokenizer = Tokenizer::new("/");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::Divide);

    // Test LeftParen
    let mut tokenizer = Tokenizer::new("(");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::LeftParen);

    // Test RightParen
    let mut tokenizer = Tokenizer::new(")");
    let token = tokenizer.next_token();
    assert_eq!(token.token_type, TokenType::RightParen);
}

/// Test operator at end of input to ensure return path is exercised
#[test]
fn test_operator_at_end() {
    let mut tokenizer = Tokenizer::new("123+");

    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Integer(123));

    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Plus);

    let token3 = tokenizer.next_token();
    assert_eq!(token3.token_type, TokenType::Eof);
}

/// Test mixed operators and whitespace at boundaries
#[test]
fn test_operators_with_trailing_whitespace() {
    let mut tokenizer = Tokenizer::new("+ ");

    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Plus);

    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Eof);

    // Call again to exercise advance() None branch
    let token3 = tokenizer.next_token();
    assert_eq!(token3.token_type, TokenType::Eof);
}

/// Test parentheses to ensure all operator branches are covered
#[test]
fn test_parentheses_return_path() {
    let mut tokenizer = Tokenizer::new("()");

    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::LeftParen);

    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::RightParen);

    let token3 = tokenizer.next_token();
    assert_eq!(token3.token_type, TokenType::Eof);
}

/// Test that exercises advance beyond input length
#[test]
fn test_exhaustive_advancement() {
    let mut tokenizer = Tokenizer::new("1");

    // Get the integer (advances once)
    let token1 = tokenizer.next_token();
    assert_eq!(token1.token_type, TokenType::Integer(1));

    // Get EOF (tries to advance but already at end)
    let token2 = tokenizer.next_token();
    assert_eq!(token2.token_type, TokenType::Eof);

    // Try multiple times to ensure the None branch is hit
    for _ in 0..5 {
        let token = tokenizer.next_token();
        assert_eq!(token.token_type, TokenType::Eof);
    }
}
