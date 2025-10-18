use grit::lexer::{Token, TokenType, Tokenizer};
use grit::parser::{ParseError, Parser};

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

    // Now includes newline token
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Newline);
    assert_eq!(tokens[1].token_type, grit::lexer::TokenType::Eof);
}

#[test]
fn test_trailing_whitespace() {
    let mut tokenizer = Tokenizer::new("42   \n  ");
    let tokens = tokenizer.tokenize();

    // Now includes newline token
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].token_type, grit::lexer::TokenType::Integer(42));
    assert_eq!(tokens[1].token_type, grit::lexer::TokenType::Newline);
    assert_eq!(tokens[2].token_type, grit::lexer::TokenType::Eof);
}

// Parser error tests

#[test]
fn test_parse_error_display_unexpected_token() {
    let token = Token {
        token_type: TokenType::Integer(42),
        line: 10,
        column: 5,
    };
    let err = ParseError::UnexpectedToken {
        expected: "identifier".to_string(),
        found: token,
    };
    let message = err.to_string();
    assert!(message.contains("line 10"));
    assert!(message.contains("column 5"));
    assert!(message.contains("identifier"));
}

#[test]
fn test_parse_error_display_unexpected_eof() {
    let err = ParseError::UnexpectedEof {
        expected: "expression".to_string(),
    };
    let message = err.to_string();
    assert!(message.contains("Unexpected end of file"));
    assert!(message.contains("expression"));
}

#[test]
fn test_class_definition_missing_name() {
    let input = "class 123"; // Number instead of class name
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ParseError::UnexpectedToken { .. }
    ));
}

#[test]
fn test_class_definition_eof_after_class_keyword() {
    let input = "class"; // EOF after class keyword
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_method_definition_missing_method_name() {
    let input = "fn MyClass > 123()"; // Number instead of method name
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_method_definition_eof_after_arrow() {
    let input = "fn MyClass >"; // EOF after >
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_function_params_unexpected_eof() {
    let input = "fn test(a, b"; // Missing closing paren
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_function_params_unexpected_token() {
    let input = "fn test(a, 123)"; // Number instead of param name
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_if_statement_missing_brace() {
    let input = "if x > 5 print('hi')"; // Missing {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ParseError::UnexpectedToken { .. }
    ));
}

#[test]
fn test_if_statement_eof_in_body() {
    let input = "if x > 5 { print('hi')"; // Missing }
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_elif_missing_brace() {
    let input = "if x > 5 { print('a') } elif x < 2 print('b')"; // Missing { after elif
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_elif_eof_in_body() {
    let input = "if x > 5 { print('a') } elif x < 2 { print('b'"; // Missing }
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_else_missing_brace() {
    let input = "if x > 5 { print('a') } else print('b')"; // Missing { after else
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}

#[test]
fn test_while_loop_missing_brace() {
    let input = "while x < 10 x = x + 1"; // Missing {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ParseError::UnexpectedToken { .. }
    ));
}

#[test]
fn test_while_loop_eof_in_body() {
    let input = "while x < 10 { x = x + 1"; // Missing }
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err());
}
