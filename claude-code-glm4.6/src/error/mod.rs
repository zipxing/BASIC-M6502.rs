//! Error handling for the BASIC interpreter
//!
//! This module defines all error types that can occur during
//! BASIC program execution, following the original Microsoft
//! BASIC error codes and messages.

use thiserror::Error;

/// All possible BASIC errors, corresponding to the original error messages
#[derive(Debug, Clone, PartialEq, Error)]
pub enum BasicError {
    /// NF - NEXT WITHOUT FOR
    #[error("NEXT WITHOUT FOR")]
    NextWithoutFor,

    /// SN - SYNTAX ERROR
    #[error("SYNTAX ERROR")]
    Syntax,

    /// RG - RETURN WITHOUT GOSUB
    #[error("RETURN WITHOUT GOSUB")]
    ReturnWithoutGosub,

    /// OD - OUT OF DATA
    #[error("OUT OF DATA")]
    OutOfData,

    /// FC - ILLEGAL QUANTITY
    #[error("ILLEGAL QUANTITY")]
    IllegalQuantity,

    /// OV - OVERFLOW
    #[error("OVERFLOW")]
    Overflow,

    /// OM - OUT OF MEMORY
    #[error("OUT OF MEMORY")]
    OutOfMemory,

    /// US - UNDEFINED STATEMENT
    #[error("UNDEFINED STATEMENT")]
    UndefinedStatement,

    /// BS - BAD SUBSCRIPT
    #[error("BAD SUBSCRIPT")]
    BadSubscript,

    /// DD - REDIMENSIONED ARRAY
    #[error("REDIMENSIONED ARRAY")]
    RedimensionedArray,

    /// /0 - DIVISION BY ZERO
    #[error("DIVISION BY ZERO")]
    DivisionByZero,

    /// ID - ILLEGAL DIRECT
    #[error("ILLEGAL DIRECT")]
    IllegalDirect,

    /// TM - TYPE MISMATCH
    #[error("TYPE MISMATCH")]
    TypeMismatch,

    /// LS - STRING TOO LONG
    #[error("STRING TOO LONG")]
    StringTooLong,

    /// FD - FILE DATA (for extended I/O)
    #[error("FILE DATA")]
    FileData,

    /// ST - STRING FORMULA TOO COMPLEX
    #[error("STRING FORMULA TOO COMPLEX")]
    StringFormulaTooComplex,

    /// CN - CAN'T CONTINUE
    #[error("CAN'T CONTINUE")]
    CantContinue,

    /// UF - UNDEFINED FUNCTION
    #[error("UNDEFINED FUNCTION")]
    UndefinedFunction,

    /// Lexer-specific errors
    #[error("Invalid number: {0}")]
    InvalidNumber(String),

    #[error("Invalid string: {0}")]
    InvalidString(String),

    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),

    /// Evaluator-specific errors
    #[error("Expected expression")]
    ExpectedExpression,

    #[error("Expected right parenthesis")]
    ExpectedRightParen,

    #[error("Unexpected token: {:?}", _0)]
    UnexpectedToken(crate::lexer::Token),

    /// Runtime errors
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Array not found: {0}")]
    ArrayNotFound(String),

    #[error("Line number not found: {0}")]
    LineNumberNotFound(u16),

    /// IO errors
    #[error("Input error: {0}")]
    InputError(String),

    /// Control flow jumps (not really errors, but used for control flow)
    #[error("GOTO jump to line: {0}")]
    GotoJump(u16),
    
    /// Jump to specific line and statement (for single-line FOR loops)
    #[error("GOTO jump to line: {0}, statement: {1}")]
    GotoJumpWithStatement(u16, usize),

    #[error("GOSUB jump to line: {0}")]
    GosubJump(u16),

    #[error("RETURN jump to line: {0}")]
    ReturnJump(u16),

    #[error("Generic error: {0}")]
    Generic(String),
}

impl BasicError {
    /// Get the original error code (as used in Microsoft BASIC)
    pub fn code(&self) -> &'static str {
        match self {
            BasicError::NextWithoutFor => "NF",
            BasicError::Syntax => "SN",
            BasicError::ReturnWithoutGosub => "RG",
            BasicError::OutOfData => "OD",
            BasicError::IllegalQuantity => "FC",
            BasicError::Overflow => "OV",
            BasicError::OutOfMemory => "OM",
            BasicError::UndefinedStatement => "US",
            BasicError::BadSubscript => "BS",
            BasicError::RedimensionedArray => "DD",
            BasicError::DivisionByZero => "/0",
            BasicError::IllegalDirect => "ID",
            BasicError::TypeMismatch => "TM",
            BasicError::StringTooLong => "LS",
            BasicError::FileData => "FD",
            BasicError::StringFormulaTooComplex => "ST",
            BasicError::CantContinue => "CN",
            BasicError::UndefinedFunction => "UF",
            _ => "GE", // Generic Error
        }
    }

    /// Check if this error can be recovered from with CONT
    pub fn can_continue(&self) -> bool {
        !matches!(
            self,
            BasicError::Syntax |
            BasicError::OutOfMemory |
            BasicError::UndefinedStatement
        )
    }
}

pub type BasicResult<T> = Result<T, BasicError>;