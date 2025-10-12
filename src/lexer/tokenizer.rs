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
    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;

            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }

            Some(ch)
        } else {
            None
        }
    }

    /// Skips whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
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
                } else {
                    self.advance();
                    let token_type = match ch {
                        '+' => TokenType::Plus,
                        '-' => TokenType::Minus,
                        '*' => TokenType::Multiply,
                        '/' => TokenType::Divide,
                        '(' => TokenType::LeftParen,
                        ')' => TokenType::RightParen,
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
