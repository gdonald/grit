use grit::lexer::{TokenType, Tokenizer};

#[test]
fn test_tokenize_single_integer() {
    let mut tokenizer = Tokenizer::new("42");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Integer(42));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_multiple_integers() {
    let mut tokenizer = Tokenizer::new("123 456 789");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(123));
    assert_eq!(tokens[1].token_type, TokenType::Integer(456));
    assert_eq!(tokens[2].token_type, TokenType::Integer(789));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_addition() {
    let mut tokenizer = Tokenizer::new("1 + 2");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(1));
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Integer(2));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_subtraction() {
    let mut tokenizer = Tokenizer::new("10 - 5");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(10));
    assert_eq!(tokens[1].token_type, TokenType::Minus);
    assert_eq!(tokens[2].token_type, TokenType::Integer(5));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_multiplication() {
    let mut tokenizer = Tokenizer::new("3 * 4");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(3));
    assert_eq!(tokens[1].token_type, TokenType::Multiply);
    assert_eq!(tokens[2].token_type, TokenType::Integer(4));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_division() {
    let mut tokenizer = Tokenizer::new("20 / 4");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(20));
    assert_eq!(tokens[1].token_type, TokenType::Divide);
    assert_eq!(tokens[2].token_type, TokenType::Integer(4));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_all_operators() {
    let mut tokenizer = Tokenizer::new("1 + 2 - 3 * 4 / 5");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].token_type, TokenType::Integer(1));
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Integer(2));
    assert_eq!(tokens[3].token_type, TokenType::Minus);
    assert_eq!(tokens[4].token_type, TokenType::Integer(3));
    assert_eq!(tokens[5].token_type, TokenType::Multiply);
    assert_eq!(tokens[6].token_type, TokenType::Integer(4));
    assert_eq!(tokens[7].token_type, TokenType::Divide);
    assert_eq!(tokens[8].token_type, TokenType::Integer(5));
    assert_eq!(tokens[9].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_parentheses() {
    let mut tokenizer = Tokenizer::new("(1 + 2)");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Integer(1));
    assert_eq!(tokens[2].token_type, TokenType::Plus);
    assert_eq!(tokens[3].token_type, TokenType::Integer(2));
    assert_eq!(tokens[4].token_type, TokenType::RightParen);
    assert_eq!(tokens[5].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_nested_parentheses() {
    let mut tokenizer = Tokenizer::new("((1 + 2) * 3)");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 10);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(tokens[2].token_type, TokenType::Integer(1));
    assert_eq!(tokens[3].token_type, TokenType::Plus);
    assert_eq!(tokens[4].token_type, TokenType::Integer(2));
    assert_eq!(tokens[5].token_type, TokenType::RightParen);
    assert_eq!(tokens[6].token_type, TokenType::Multiply);
    assert_eq!(tokens[7].token_type, TokenType::Integer(3));
    assert_eq!(tokens[8].token_type, TokenType::RightParen);
    assert_eq!(tokens[9].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_no_whitespace() {
    let mut tokenizer = Tokenizer::new("1+2*3");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].token_type, TokenType::Integer(1));
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Integer(2));
    assert_eq!(tokens[3].token_type, TokenType::Multiply);
    assert_eq!(tokens[4].token_type, TokenType::Integer(3));
    assert_eq!(tokens[5].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_complex_expression() {
    let mut tokenizer = Tokenizer::new("(10 + 20) * (30 - 15) / 5");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 14);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Integer(10));
    assert_eq!(tokens[2].token_type, TokenType::Plus);
    assert_eq!(tokens[3].token_type, TokenType::Integer(20));
    assert_eq!(tokens[4].token_type, TokenType::RightParen);
    assert_eq!(tokens[5].token_type, TokenType::Multiply);
    assert_eq!(tokens[6].token_type, TokenType::LeftParen);
    assert_eq!(tokens[7].token_type, TokenType::Integer(30));
    assert_eq!(tokens[8].token_type, TokenType::Minus);
    assert_eq!(tokens[9].token_type, TokenType::Integer(15));
    assert_eq!(tokens[10].token_type, TokenType::RightParen);
    assert_eq!(tokens[11].token_type, TokenType::Divide);
    assert_eq!(tokens[12].token_type, TokenType::Integer(5));
    assert_eq!(tokens[13].token_type, TokenType::Eof);
}
