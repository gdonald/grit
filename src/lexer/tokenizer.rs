use super::token::{Token, TokenType};

/// Tokenizer for the Grit language
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Tokenizer {
    /// Creates a new tokenizer from the given input string
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Returns the current character without consuming it
    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    /// Advances to the next character and returns it
    ///
    /// # Safety
    /// This method assumes position < input.len().
    /// Always call current_char() first to check if there are more characters.
    /// Calling this method when position >= input.len() will cause a panic.
    fn advance(&mut self) -> char {
        let ch = self.input[self.position];
        self.position += 1;

        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        ch
    }

    /// Skips whitespace characters (excluding newlines)
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() && ch != '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Reads an identifier or keyword from the input
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    /// Reads a string literal from the input (single-quoted)
    fn read_string(&mut self) -> String {
        let mut string = String::new();
        self.advance(); // consume opening quote

        while let Some(ch) = self.current_char() {
            if ch == '\'' {
                self.advance(); // consume closing quote
                break;
            } else if ch == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '\'' => string.push('\''),
                        _ => {
                            string.push('\\');
                            string.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }

        string
    }

    /// Reads an integer from the input
    fn read_integer(&mut self) -> i64 {
        let mut number = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_ascii_digit() {
                number.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        number.parse().unwrap_or(0)
    }

    /// Returns the next token from the input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let line = self.line;
        let column = self.column;

        match self.current_char() {
            None => Token::new(TokenType::Eof, line, column),
            Some(ch) => {
                if ch.is_ascii_digit() {
                    let number = self.read_integer();
                    Token::new(TokenType::Integer(number), line, column)
                } else if ch.is_alphabetic() || ch == '_' {
                    let identifier = self.read_identifier();
                    let token_type = match identifier.as_str() {
                        "fn" => TokenType::Fn,
                        "if" => TokenType::If,
                        "elif" => TokenType::Elif,
                        "else" => TokenType::Else,
                        "while" => TokenType::While,
                        _ => TokenType::Identifier(identifier),
                    };
                    Token::new(token_type, line, column)
                } else if ch == '\'' {
                    let string = self.read_string();
                    Token::new(TokenType::String(string), line, column)
                } else {
                    self.advance();
                    let token_type = match ch {
                        '+' => TokenType::Plus,
                        '-' => TokenType::Minus,
                        '*' => TokenType::Multiply,
                        '/' => TokenType::Divide,
                        '=' => {
                            // Check for ==
                            if self.current_char() == Some('=') {
                                self.advance();
                                TokenType::EqualEqual
                            } else {
                                TokenType::Equals
                            }
                        }
                        '!' => {
                            // Check for !=
                            if self.current_char() == Some('=') {
                                self.advance();
                                TokenType::NotEqual
                            } else {
                                panic!(
                                    "Unexpected character '{}' at line {}, column {}",
                                    ch, line, column
                                )
                            }
                        }
                        '<' => {
                            // Check for <=
                            if self.current_char() == Some('=') {
                                self.advance();
                                TokenType::LessThanOrEqual
                            } else {
                                TokenType::LessThan
                            }
                        }
                        '>' => {
                            // Check for >=
                            if self.current_char() == Some('=') {
                                self.advance();
                                TokenType::GreaterThanOrEqual
                            } else {
                                TokenType::GreaterThan
                            }
                        }
                        '(' => TokenType::LeftParen,
                        ')' => TokenType::RightParen,
                        '{' => TokenType::LeftBrace,
                        '}' => TokenType::RightBrace,
                        ',' => TokenType::Comma,
                        '\n' => TokenType::Newline,
                        _ => {
                            panic!(
                                "Unexpected character '{}' at line {}, column {}",
                                ch, line, column
                            )
                        }
                    };
                    Token::new(token_type, line, column)
                }
            }
        }
    }

    /// Tokenizes the entire input and returns a vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token();
            let is_eof = token.token_type == TokenType::Eof;
            tokens.push(token);

            if is_eof {
                break;
            }
        }

        tokens
    }
}
