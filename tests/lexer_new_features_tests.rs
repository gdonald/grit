use grit::lexer::{TokenType, Tokenizer};

#[test]
fn test_tokenize_identifier() {
    let mut tokenizer = Tokenizer::new("abc");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("abc".to_string())
    );
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_identifier_with_underscore() {
    let mut tokenizer = Tokenizer::new("my_var");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("my_var".to_string())
    );
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_identifier_with_numbers() {
    let mut tokenizer = Tokenizer::new("var123");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("var123".to_string())
    );
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
    assert_eq!(
        tokens[0].token_type,
        TokenType::String("hello\nworld".to_string())
    );
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
    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("print".to_string())
    );
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
    assert_eq!(
        tokens[10].token_type,
        TokenType::Identifier("a".to_string())
    );
}

// Float tokenization edge case tests

#[test]
fn test_tokenize_float_zero() {
    let mut tokenizer = Tokenizer::new("0.0");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Float(0.0));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_float_with_many_decimals() {
    let mut tokenizer = Tokenizer::new("3.14159265359");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Float(3.14159265359));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_float_starting_with_zero() {
    let mut tokenizer = Tokenizer::new("0.5");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, TokenType::Float(0.5));
    assert_eq!(tokens[1].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_multiple_floats() {
    let mut tokenizer = Tokenizer::new("1.5 + 2.3");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Float(1.5));
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Float(2.3));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_float_vs_method_call() {
    // "42.foo" should tokenize as Integer, Dot, Identifier
    let mut tokenizer = Tokenizer::new("42.foo");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(42));
    assert_eq!(tokens[1].token_type, TokenType::Dot);
    assert_eq!(
        tokens[2].token_type,
        TokenType::Identifier("foo".to_string())
    );
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_float_at_end() {
    // "42." at end of input should be Integer then Dot
    let mut tokenizer = Tokenizer::new("42.");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, TokenType::Integer(42));
    assert_eq!(tokens[1].token_type, TokenType::Dot);
    assert_eq!(tokens[2].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_float_assignment() {
    let mut tokenizer = Tokenizer::new("pi = 3.14");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(
        tokens[0].token_type,
        TokenType::Identifier("pi".to_string())
    );
    assert_eq!(tokens[1].token_type, TokenType::Equals);
    assert_eq!(tokens[2].token_type, TokenType::Float(3.14));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_float_in_expression() {
    let mut tokenizer = Tokenizer::new("2.5 * (1.0 + 3.5)");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 8);
    assert_eq!(tokens[0].token_type, TokenType::Float(2.5));
    assert_eq!(tokens[1].token_type, TokenType::Multiply);
    assert_eq!(tokens[2].token_type, TokenType::LeftParen);
    assert_eq!(tokens[3].token_type, TokenType::Float(1.0));
    assert_eq!(tokens[4].token_type, TokenType::Plus);
    assert_eq!(tokens[5].token_type, TokenType::Float(3.5));
    assert_eq!(tokens[6].token_type, TokenType::RightParen);
    assert_eq!(tokens[7].token_type, TokenType::Eof);
}

#[test]
fn test_tokenize_mixed_int_and_float() {
    let mut tokenizer = Tokenizer::new("5 + 2.5");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].token_type, TokenType::Integer(5));
    assert_eq!(tokens[1].token_type, TokenType::Plus);
    assert_eq!(tokens[2].token_type, TokenType::Float(2.5));
    assert_eq!(tokens[3].token_type, TokenType::Eof);
}
