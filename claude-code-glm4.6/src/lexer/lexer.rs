//! Lexical analyzer for BASIC
//!
//! This module implements the lexer that converts text input into tokens,
//! corresponding to the CRUNCH process in the original Microsoft BASIC.

use super::tokens::{Token, keyword_to_token};
use crate::error::{BasicError, BasicResult};

/// BASIC lexical analyzer
pub struct Lexer {
    /// Current position in input
    position: usize,
    /// Current character
    current_char: Option<char>,
    /// Input text
    input: String,
}

impl Lexer {
    /// Create a new lexer
    pub fn new() -> Self {
        Self {
            position: 0,
            current_char: None,
            input: String::new(),
        }
    }

    /// Tokenize input string into tokens
    pub fn tokenize(&mut self, input: &str) -> BasicResult<Vec<Token>> {
        self.input = input.trim().to_string();
        self.position = 0;
        self.current_char = self.input.chars().next();

        let mut tokens = Vec::new();

        // Check for line number at the beginning
        if let Some(line_number) = self.try_parse_line_number()? {
            tokens.push(Token::LineNumber(line_number));
        }

        // Skip whitespace after line number
        self.skip_whitespace();

        // Tokenize the rest of the line
        while let Some(c) = self.current_char {
            match c {
                '"' => tokens.push(self.parse_string()?),
                '0'..='9' | '.' => tokens.push(self.parse_number()?),
                'A'..='Z' | 'a'..='z' => {
                    // Try to parse as keyword first, then as identifier
                    let identifier = self.parse_identifier()?;

                    // Special handling for REM - everything after REM is ignored
                    if identifier == "REM" {
                        // Add REM token and skip rest of line - REM is a comment
                        tokens.push(Token::Rem);
                        break;
                    } else if let Some(keyword) = keyword_to_token(&identifier) {
                        tokens.push(keyword);
                    } else {
                        // Check if it's a function name ending with $
                        if identifier.ends_with('$') {
                            let base_name = &identifier[..identifier.len()-1];
                            if let Some(func_token) = keyword_to_token(&format!("{}$", base_name)) {
                                tokens.push(func_token);
                            } else {
                                tokens.push(Token::Identifier(identifier));
                            }
                        } else {
                            tokens.push(Token::Identifier(identifier));
                        }
                    }
                }
                '+' => { tokens.push(Token::Plus); self.advance(); }
                '-' => { tokens.push(Token::Minus); self.advance(); }
                '*' => { tokens.push(Token::Multiply); self.advance(); }
                '/' => { tokens.push(Token::Divide); self.advance(); }
                '^' => { tokens.push(Token::Power); self.advance(); }
                '=' => { tokens.push(Token::Equal); self.advance(); }
                '<' => {
                    self.advance();
                    if self.current_char == Some('>') {
                        tokens.push(Token::NotEqual);
                        self.advance();
                    } else if self.current_char == Some('=') {
                        tokens.push(Token::LessEqual);
                        self.advance();
                    } else {
                        tokens.push(Token::Less);
                    }
                }
                '>' => {
                    self.advance();
                    if self.current_char == Some('=') {
                        tokens.push(Token::GreaterEqual);
                        self.advance();
                    } else {
                        tokens.push(Token::Greater);
                    }
                }
                ',' => { tokens.push(Token::Comma); self.advance(); }
                ';' => { tokens.push(Token::Semicolon); self.advance(); }
                ':' => { tokens.push(Token::Colon); self.advance(); }
                '(' => { tokens.push(Token::LeftParen); self.advance(); }
                ')' => { tokens.push(Token::RightParen); self.advance(); }
                '?' => { tokens.push(Token::Question); self.advance(); }
                ' ' | '\t' => self.skip_whitespace(),
                _ => return Err(BasicError::UnexpectedCharacter(c)),
            }
        }

        Ok(tokens)
    }

    /// Try to parse a line number at the current position
    fn try_parse_line_number(&mut self) -> BasicResult<Option<u16>> {
        let start_pos = self.position;

        // Check if we start with digits
        if !self.current_char.map_or(false, |c| c.is_ascii_digit()) {
            return Ok(None);
        }

        // Parse all digits
        let mut number_str = String::new();
        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                number_str.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if followed by whitespace (indicating a line number)
        if self.current_char.map_or(false, |c| c.is_whitespace()) {
            let line_number = number_str.parse::<u16>()
                .map_err(|_| BasicError::IllegalQuantity)?;
            Ok(Some(line_number))
        } else {
            // Not a line number, backtrack
            self.position = start_pos;
            self.current_char = self.input.chars().nth(self.position);
            Ok(None)
        }
    }

    /// Parse a string literal
    fn parse_string(&mut self) -> BasicResult<Token> {
        self.advance(); // Skip opening quote

        let mut string_value = String::new();

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance(); // Skip closing quote
                break;
            } else if c == '\\' {
                // Handle escape sequences
                self.advance();
                if let Some(escaped) = self.current_char {
                    match escaped {
                        'n' => string_value.push('\n'),
                        't' => string_value.push('\t'),
                        'r' => string_value.push('\r'),
                        '\\' => string_value.push('\\'),
                        '"' => string_value.push('"'),
                        _ => string_value.push(escaped),
                    }
                    self.advance();
                }
            } else {
                string_value.push(c);
                self.advance();
            }
        }

        Ok(Token::String(string_value))
    }

    /// Parse a number (integer or float)
    fn parse_number(&mut self) -> BasicResult<Token> {
        let mut number_str = String::new();
        let mut has_decimal = false;
        let mut has_exponent = false;

        while let Some(c) = self.current_char {
            match c {
                '0'..='9' => {
                    number_str.push(c);
                    self.advance();
                }
                '.' if !has_decimal && !has_exponent => {
                    number_str.push(c);
                    has_decimal = true;
                    self.advance();
                }
                'e' | 'E' if !has_exponent => {
                    number_str.push(c);
                    has_exponent = true;
                    self.advance();

                    // Handle optional sign after exponent
                    if let Some('+') | Some('-') = self.current_char {
                        number_str.push(self.current_char.unwrap());
                        self.advance();
                    }
                }
                _ => break,
            }
        }

        // Parse as f64 since BASIC uses floating point for all numbers
        let value = number_str.parse::<f64>()
            .map_err(|_| BasicError::InvalidNumber(number_str.clone()))?;

        Ok(Token::Number(value))
    }

    /// Parse an identifier or keyword
    fn parse_identifier(&mut self) -> BasicResult<String> {
        let mut identifier = String::new();

        while let Some(c) = self.current_char {
            match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '$' | '%' => {
                    identifier.push(c.to_ascii_uppercase());
                    self.advance();
                }
                _ => break,
            }
        }

        Ok(identifier)
    }

    /// Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Advance to the next character
    fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.chars().nth(self.position);
    }

    /// Peek at the next character without advancing
    #[allow(dead_code)]
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    /// Peek at the character after next
    #[allow(dead_code)]
    fn peek_next(&self) -> Option<char> {
        self.input.chars().nth(self.position + 2)
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("PRINT \"HELLO\"").unwrap();
        assert_eq!(tokens, vec![
            Token::Print,
            Token::String("HELLO".to_string()),
        ]);
    }

    #[test]
    fn test_tokenize_number() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("3.14").unwrap();
        assert_eq!(tokens, vec![Token::Number(3.14)]);
    }

    #[test]
    fn test_tokenize_line_number() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("10 PRINT X").unwrap();
        assert_eq!(tokens, vec![
            Token::LineNumber(10),
            Token::Print,
            Token::Identifier("X".to_string()),
        ]);
    }

    #[test]
    fn test_tokenize_expressions() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("X + Y * 2").unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier("X".to_string()),
            Token::Plus,
            Token::Identifier("Y".to_string()),
            Token::Multiply,
            Token::Number(2.0),
        ]);
    }

    #[test]
    fn test_tokenize_operators() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("A <= B AND C <> D").unwrap();
        assert_eq!(tokens, vec![
            Token::Identifier("A".to_string()),
            Token::LessEqual,
            Token::Identifier("B".to_string()),
            Token::And,
            Token::Identifier("C".to_string()),
            Token::NotEqual,
            Token::Identifier("D".to_string()),
        ]);
    }

    #[test]
    fn test_tokenize_string_functions() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("LEN(A$) + LEFT$(B$, 3)").unwrap();
        assert_eq!(tokens, vec![
            Token::Len,
            Token::LeftParen,
            Token::Identifier("A$".to_string()),
            Token::RightParen,
            Token::Plus,
            Token::Left,
            Token::LeftParen,
            Token::Identifier("B$".to_string()),
            Token::Comma,
            Token::Number(3.0),
            Token::RightParen,
        ]);
    }

    #[test]
    fn test_rem_comment() {
        let mut lexer = Lexer::new();

        // Test that REM and everything after it is ignored
        let tokens = lexer.tokenize("10 PRINT HELLO").unwrap();
        assert_eq!(tokens, vec![
            Token::LineNumber(10),
            Token::Print,
            Token::Identifier("HELLO".to_string()),
        ]);

        // Test REM with content after it - should only tokenize up to REM
        let tokens = lexer.tokenize("10 REM This is a comment PRINT \"THIS WON'T PRINT\"").unwrap();
        assert_eq!(tokens, vec![
            Token::LineNumber(10),
            Token::Rem,
        ]);

        // Test REM at beginning without line number
        let tokens = lexer.tokenize("REM This entire line is a comment").unwrap();
        assert_eq!(tokens, vec![
            Token::Rem,
        ]);

        // Test REM with mixed case
        let tokens = lexer.tokenize("ReM This is also a comment").unwrap();
        assert_eq!(tokens, vec![
            Token::Rem,
        ]);
    }

    #[test]
    fn test_tokenize_question_mark() {
        let mut lexer = Lexer::new();

        let tokens = lexer.tokenize("?X").unwrap();
        assert_eq!(tokens, vec![
            Token::Question,
            Token::Identifier("X".to_string()),
        ]);
    }
}