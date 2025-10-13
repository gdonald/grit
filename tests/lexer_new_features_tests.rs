use grit::lexer::{TokenType, Tokenizer};

#[test]
fn test_tokenize_identifier() {
    let mut tokenizer = Tokenizer::new("abc");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("abc".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_identifier_with_underscore() {
    let mut tokenizer = Tokenizer::new("my_var");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("my_var".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_identifier_with_numbers() {
    let mut tokenizer = Tokenizer::new("var123");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("var123".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_string_literal() {
    let mut tokenizer = Tokenizer::new("'hello'");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String("hello".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_string_with_escape_sequences() {
    let mut tokenizer = Tokenizer::new("'hello\\nworld'");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String("hello\nworld".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_string_with_tab() {
    let mut tokenizer = Tokenizer::new("'a\\tb'");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String("a\tb".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_string_with_backslash() {
    let mut tokenizer = Tokenizer::new("'a\\\\b'");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String("a\\b".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_string_with_escaped_quote() {
    let mut tokenizer = Tokenizer::new("'don\\'t'");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String("don't".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_string_with_unknown_escape() {
    let mut tokenizer = Tokenizer::new("'a\\xb'");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::String("a\\xb".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_equals() {
    let mut tokenizer = Tokenizer::new("=");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Equals);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_comma() {
    let mut tokenizer = Tokenizer::new(",");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Comma);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_newline() {
    let mut tokenizer = Tokenizer::new("\n");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Newline);
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_assignment() {
    let mut tokenizer = Tokenizer::new("a = 1");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("a".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Equals);
    assert_eq!(tokens[2].token_type, TokenType::Integer(1));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_function_call() {
    let mut tokenizer = Tokenizer::new("print('hello', 42)");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("print".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::LeftParen);
    assert_eq!(tokens[2].token_type, TokenType::String("hello".to_string()));
    assert_eq!(tokens[3].token_type, TokenType::Comma);
    assert_eq!(tokens[4].token_type, TokenType::Integer(42));
    assert_eq!(tokens[5].token_type, TokenType::RightParen);
    assert_eq!(tokens[6].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_multiline_program() {
    let mut tokenizer = Tokenizer::new("a = 1\nb = 2\nc = a + b");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 14);
    assert_eq!(tokens[0].token_type, TokenType::Identifier("a".to_string()));
    assert_eq!(tokens[1].token_type, TokenType::Equals);
    assert_eq!(tokens[2].token_type, TokenType::Integer(1));
    assert_eq!(tokens[3].token_type, TokenType::Newline);
    assert_eq!(tokens[4].token_type, TokenType::Identifier("b".to_string()));
    assert_eq!(tokens[5].token_type, TokenType::Equals);
    assert_eq!(tokens[6].token_type, TokenType::Integer(2));
    assert_eq!(tokens[7].token_type, TokenType::Newline);
    assert_eq!(tokens[8].token_type, TokenType::Identifier("c".to_string()));
    assert_eq!(tokens[9].token_type, TokenType::Equals);
    assert_eq!(tokens[10].token_type, TokenType::Identifier("a".to_string()));
}
