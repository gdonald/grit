use grit::lexer::Tokenizer;

#[test]
#[should_panic(expected = "Unexpected character")]
fn test_unexpected_character() {
    let mut tokenizer = Tokenizer::new("1 + @");
    tokenizer.tokenize();
}

#[test]
#[should_panic(expected = "Unexpected character '$'")]
fn test_unexpected_character_dollar() {
    let mut tokenizer = Tokenizer::new("$");
    tokenizer.tokenize();
}

#[test]
#[should_panic(expected = "Unexpected character '#'")]
fn test_unexpected_character_hash() {
    let mut tokenizer = Tokenizer::new("5 # 3");
    tokenizer.tokenize();
}

#[test]
#[should_panic(expected = "Unexpected character '&'")]
fn test_unexpected_character_ampersand() {
    let mut tokenizer = Tokenizer::new("10 & 20");
    tokenizer.tokenize();
}

#[test]
#[should_panic(expected = "Unexpected character '!'")]
fn test_unexpected_character_exclamation() {
    let mut tokenizer = Tokenizer::new("!");
    tokenizer.tokenize();
}

#[test]
fn test_empty_input() {
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Eof);
}

#[test]
fn test_only_whitespace() {
    let mut tokenizer = Tokenizer::new("   \n\t  ");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Eof);
}

#[test]
fn test_trailing_whitespace() {
    let mut tokenizer = Tokenizer::new("42   \n  ");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Integer(42));
    assert_eq!(tokens[1].token_type, grit::lexer::TokenType::Eof);
}
