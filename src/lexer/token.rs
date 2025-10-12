/// Represents the different types of tokens in the Grit language
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Integer(i64),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,

    // Delimiters
    LeftParen,
    RightParen,

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
