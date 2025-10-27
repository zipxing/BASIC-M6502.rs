use thiserror::Error;

/// Unified error type with BASIC-like short messages.
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum BasicError {
    #[error("SYNTAX ERROR")]
    Syntax,
    #[error("TYPE MISMATCH")]
    TypeMismatch,
    #[error("NEXT WITHOUT FOR")]
    NextWithoutFor,
    #[error("RETURN WITHOUT GOSUB")]
    ReturnWithoutGosub,
    #[error("UNDEFINED VARIABLE: {0}")]
    UndefVar(String),
    #[error("OUT OF DATA")]
    OutOfData,
    #[error("BAD SUBSCRIPT")]
    BadSubscript,
    #[error("UNDEFINED ARRAY")]
    UndefinedArray,
    #[error("IO ERROR")]
    Io,
}
