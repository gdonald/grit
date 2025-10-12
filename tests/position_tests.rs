use grit::lexer::{TokenType, Tokenizer};

#[test]
fn test_token_positions_single_line() {
    let mut tokenizer = Tokenizer::new("1 + 2");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);

    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 3);

    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 5);
}

#[test]
fn test_token_positions_multiple_lines() {
    let mut tokenizer = Tokenizer::new("1 + 2\n3 * 4");
    let tokens = tokenizer.tokenize();

    // Line 1: "1 + 2"
    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);
    assert_eq!(tokens[0].token_type, TokenType::Integer(1));

    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 3);
    assert_eq!(tokens[1].token_type, TokenType::Plus);

    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 5);
    assert_eq!(tokens[2].token_type, TokenType::Integer(2));

    // Line 2: "3 * 4"
    assert_eq!(tokens[3].line, 2);
    assert_eq!(tokens[3].column, 1);
    assert_eq!(tokens[3].token_type, TokenType::Integer(3));

    assert_eq!(tokens[4].line, 2);
    assert_eq!(tokens[4].column, 3);
    assert_eq!(tokens[4].token_type, TokenType::Multiply);

    assert_eq!(tokens[5].line, 2);
    assert_eq!(tokens[5].column, 5);
    assert_eq!(tokens[5].token_type, TokenType::Integer(4));
}

#[test]
fn test_token_positions_with_parentheses() {
    let mut tokenizer = Tokenizer::new("(10)");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens[0].line, 1);
    assert_eq!(tokens[0].column, 1);
    assert_eq!(tokens[0].token_type, TokenType::LeftParen);

    assert_eq!(tokens[1].line, 1);
    assert_eq!(tokens[1].column, 2);
    assert_eq!(tokens[1].token_type, TokenType::Integer(10));

    assert_eq!(tokens[2].line, 1);
    assert_eq!(tokens[2].column, 4);
    assert_eq!(tokens[2].token_type, TokenType::RightParen);
}
