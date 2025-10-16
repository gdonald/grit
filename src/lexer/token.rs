/// Represents the different types of tokens in the Grit language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),
    String(String),
    Identifier(String),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,

    // Comparison operators
    EqualEqual,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Newline,
    Dot,

    // Keywords
    Fn,
    If,
    Elif,
    Else,
    While,
    Class,
    Self_,

    // Special
    Eof,
}

/// Represents a token with its type and position in the source code
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new token
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Token {
            token_type,
            line,
            column,
        }
    }
}
