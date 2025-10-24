//! Token definitions for the BASIC lexer
//!
//! This module defines all tokens used in the BASIC language,
//! corresponding to the original token system in Microsoft BASIC.

#[allow(dead_code)]

use std::fmt;

/// All possible tokens in BASIC, corresponding to the original token values
#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Statement keywords - corresponding to STMDSP table
    End,     // 128
    For,     // 129
    Next,    // 130
    Data,    // 131
    Input,   // 132
    Dim,     // 133
    Read,    // 134
    Let,     // 135
    Goto,    // 136
    Run,     // 137
    If,      // 138
    Restore, // 139
    Gosub,   // 140
    Return,  // 141
    Rem,     // 142
    Stop,    // 143
    On,      // 144
    Def,     // 150
    Poke,    // 151
    Print,   // 152
    Cont,    // 153
    List,    // 154
    Clear,   // 155
    Get,     // 160
    New,     // 161
    Load,    // 162
    Save,    // 163

    // Functions - corresponding to FUNDSP table
    Sgn,     // 164
    Int,     // 165
    Abs,     // 166
    Fre,     // 167
    Pos,     // 168
    Sqr,     // 169
    Rnd,     // 170
    Log,     // 171
    Exp,     // 172
    Cos,     // 173
    Sin,     // 174
    Tan,     // 175
    Atn,     // 176
    Peek,    // 177
    Len,     // 178
    Str,     // 179 (STR$)
    Val,     // 180
    Asc,     // 181
    Chr,     // 182 (CHR$)
    Left,    // 183 (LEFT$)
    Right,   // 184 (RIGHT$)
    Mid,     // 185 (MID$)

    // Operators - corresponding to OPTAB
    Plus,        // +
    Minus,       // -
    Multiply,    // *
    Divide,      // /
    Power,       // ^
    And,         // AND
    Or,          // OR
    Not,         // NOT
    Equal,       // =
    NotEqual,    // <> or ><
    Less,        // <
    Greater,     // >
    LessEqual,   // <= or =<
    GreaterEqual,// >= or =>

    // Punctuation and separators
    Comma,       // ,
    Semicolon,   // ;
    Colon,       // :
    LeftParen,   // (
    RightParen,  // )
    Question,    // ? (shortcut for PRINT)

    // Literals and identifiers
    Number(f64),      // Numeric literal
    String(String),   // String literal
    Identifier(String), // Variable name
    LineNumber(u16),  // Program line number

    // Special tokens
    EndOfLine,
    EndOfFile,
}

impl Token {
    /// Get the original token value (for compatibility with original BASIC)
    pub fn token_value(&self) -> u8 {
        match self {
            // Statements (128-161)
            Token::End => 128,
            Token::For => 129,
            Token::Next => 130,
            Token::Data => 131,
            Token::Input => 132,
            Token::Dim => 133,
            Token::Read => 134,
            Token::Let => 135,
            Token::Goto => 136,
            Token::Run => 137,
            Token::If => 138,
            Token::Restore => 139,
            Token::Gosub => 140,
            Token::Return => 141,
            Token::Rem => 142,
            Token::Stop => 143,
            Token::On => 144,
            Token::Def => 150,
            Token::Poke => 151,
            Token::Print => 152,
            Token::Cont => 153,
            Token::List => 154,
            Token::Clear => 155,
            Token::Get => 160,
            Token::New => 161,
            Token::Load => 162,
            Token::Save => 163,

            // Functions (162-183)
            Token::Sgn => 162,
            Token::Int => 163,
            Token::Abs => 164,
            Token::Fre => 165,
            Token::Pos => 166,
            Token::Sqr => 167,
            Token::Rnd => 168,
            Token::Log => 169,
            Token::Exp => 170,
            Token::Cos => 171,
            Token::Sin => 172,
            Token::Tan => 173,
            Token::Atn => 174,
            Token::Peek => 175,
            Token::Len => 176,
            Token::Str => 177,
            Token::Val => 178,
            Token::Asc => 179,
            Token::Chr => 180,
            Token::Left => 181,
            Token::Right => 182,
            Token::Mid => 183,

            // Other tokens don't have original values
            _ => 0,
        }
    }

    /// Check if this token is a statement keyword
    pub fn is_statement(&self) -> bool {
        matches!(
            self,
            Token::End | Token::For | Token::Next | Token::Data | Token::Input |
            Token::Dim | Token::Read | Token::Let | Token::Goto | Token::Run |
            Token::If | Token::Restore | Token::Gosub | Token::Return | Token::Rem |
            Token::Stop | Token::On | Token::Def | Token::Poke | Token::Print |
            Token::Cont | Token::List | Token::Clear | Token::Get | Token::New |
            Token::Load | Token::Save
        )
    }

    /// Check if this token is a function
    pub fn is_function(&self) -> bool {
        matches!(
            self,
            Token::Sgn | Token::Int | Token::Abs | Token::Fre | Token::Pos |
            Token::Sqr | Token::Rnd | Token::Log | Token::Exp | Token::Cos |
            Token::Sin | Token::Tan | Token::Atn | Token::Peek | Token::Len |
            Token::Str | Token::Val | Token::Asc | Token::Chr | Token::Left |
            Token::Right | Token::Mid
        )
    }

    /// Check if this token is an operator
    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide |
            Token::Power | Token::And | Token::Or | Token::Not |
            Token::Equal | Token::NotEqual | Token::Less | Token::Greater |
            Token::LessEqual | Token::GreaterEqual
        )
    }

    /// Get the precedence of this operator (higher number = higher precedence)
    pub fn precedence(&self) -> u8 {
        match self {
            Token::And => 80,
            Token::Or => 70,
            Token::Equal | Token::NotEqual | Token::Less | Token::Greater |
            Token::LessEqual | Token::GreaterEqual => 100,
            Token::Plus | Token::Minus => 121,
            Token::Multiply | Token::Divide => 123,
            Token::Power => 127,
            Token::Not => 90,
            _ => 0,
        }
    }

    /// Check if this operator is left-associative
    pub fn is_left_associative(&self) -> bool {
        !matches!(self, Token::Power) // Power is right-associative
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Identifier(s) => write!(f, "{}", s),
            Token::LineNumber(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Multiply => write!(f, "*"),
            Token::Divide => write!(f, "/"),
            Token::Power => write!(f, "^"),
            Token::Equal => write!(f, "="),
            Token::NotEqual => write!(f, "<>"),
            Token::Less => write!(f, "<"),
            Token::Greater => write!(f, ">"),
            Token::LessEqual => write!(f, "<="),
            Token::GreaterEqual => write!(f, ">="),
            Token::And => write!(f, "AND"),
            Token::Or => write!(f, "OR"),
            Token::Not => write!(f, "NOT"),
            Token::Comma => write!(f, ","),
            Token::Semicolon => write!(f, ";"),
            Token::Colon => write!(f, ":"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Question => write!(f, "?"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Convert a keyword string to its corresponding token
pub fn keyword_to_token(keyword: &str) -> Option<Token> {
    match keyword.to_uppercase().as_str() {
        "END" => Some(Token::End),
        "FOR" => Some(Token::For),
        "NEXT" => Some(Token::Next),
        "DATA" => Some(Token::Data),
        "INPUT" => Some(Token::Input),
        "DIM" => Some(Token::Dim),
        "READ" => Some(Token::Read),
        "LET" => Some(Token::Let),
        "GOTO" => Some(Token::Goto),
        "RUN" => Some(Token::Run),
        "IF" => Some(Token::If),
        "THEN" => None, // THEN is part of IF syntax
        "RESTORE" => Some(Token::Restore),
        "GOSUB" => Some(Token::Gosub),
        "RETURN" => Some(Token::Return),
        "REM" => Some(Token::Rem),
        "STOP" => Some(Token::Stop),
        "ON" => Some(Token::On),
        "DEF" => Some(Token::Def),
        "FN" => None, // FN is prefix for user functions
        "POKE" => Some(Token::Poke),
        "PRINT" => Some(Token::Print),
        "CONT" => Some(Token::Cont),
        "LIST" => Some(Token::List),
        "CLEAR" => Some(Token::Clear),
        "GET" => Some(Token::Get),
        "NEW" => Some(Token::New),
        "LOAD" => Some(Token::Load),
        "SAVE" => Some(Token::Save),
        "SGN" => Some(Token::Sgn),
        "INT" => Some(Token::Int),
        "ABS" => Some(Token::Abs),
        "FRE" => Some(Token::Fre),
        "POS" => Some(Token::Pos),
        "SQR" => Some(Token::Sqr),
        "RND" => Some(Token::Rnd),
        "LOG" => Some(Token::Log),
        "EXP" => Some(Token::Exp),
        "COS" => Some(Token::Cos),
        "SIN" => Some(Token::Sin),
        "TAN" => Some(Token::Tan),
        "ATN" => Some(Token::Atn),
        "PEEK" => Some(Token::Peek),
        "LEN" => Some(Token::Len),
        "STR$" => Some(Token::Str),
        "VAL" => Some(Token::Val),
        "ASC" => Some(Token::Asc),
        "CHR$" => Some(Token::Chr),
        "LEFT$" => Some(Token::Left),
        "RIGHT$" => Some(Token::Right),
        "MID$" => Some(Token::Mid),
        "AND" => Some(Token::And),
        "OR" => Some(Token::Or),
        "NOT" => Some(Token::Not),
        "TO" => None, // TO is part of FOR syntax
        "STEP" => None, // STEP is part of FOR syntax
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_to_token() {
        assert_eq!(keyword_to_token("PRINT"), Some(Token::Print));
        assert_eq!(keyword_to_token("print"), Some(Token::Print)); // Case insensitive
        assert_eq!(keyword_to_token("IF"), Some(Token::If));
        assert_eq!(keyword_to_token("THEN"), None); // THEN is special
    }

    #[test]
    fn test_token_properties() {
        assert!(Token::Print.is_statement());
        assert!(Token::Sin.is_function());
        assert!(Token::Plus.is_operator());
        assert!(!Token::Number(42.0).is_statement());
    }

    #[test]
    fn test_operator_precedence() {
        assert!(Token::Multiply.precedence() > Token::Plus.precedence());
        assert!(Token::Power.precedence() > Token::Multiply.precedence());
        assert!(Token::And.precedence() < Token::Plus.precedence());
    }

    #[test]
    fn test_token_values() {
        assert_eq!(Token::End.token_value(), 128);
        assert_eq!(Token::Print.token_value(), 152);
        assert_eq!(Token::Sin.token_value(), 172);
        assert_eq!(Token::Plus.token_value(), 0); // No original value
    }
}