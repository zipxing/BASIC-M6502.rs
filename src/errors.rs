use thiserror::Error;

/// Unified error type with BASIC-like short messages.
#[derive(Debug, Error)]
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
}

pub type Result<T> = std::result::Result<T, BasicError>;
