/// BASIC reserved-word token numbers typically start at 128 (high bit set).
/// We align with a common Microsoft 6502 BASIC subset and can extend later.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TokenKind {
    // Statements
    End = 128,
    For,
    Next,
    Data,
    Input,
    Dim,
    Read,
    Let,
    Goto,
    Run,
    List,
    If,
    Restore,
    Gosub,
    Return,
    Rem,
    Stop,
    On,
    Print,
    // Operators/Separators
    Then,
    To,
}

/// Token items. The crunching phase maps keywords to single-byte tokens (stored as u16 here).
#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    Keyword(TokenKind),
    Ident(String),
    Number(f64),
    String(String),
    Symbol(char), // e.g., = + - * / ( ) , ; :
}

/// Keyword lookup table (uppercase input expected).
pub fn lookup_keyword_upper(s: &str) -> Option<TokenKind> {
    use TokenKind::*;
    match s {
        "END" => Some(End),
        "FOR" => Some(For),
        "NEXT" => Some(Next),
        "DATA" => Some(Data),
        "INPUT" => Some(Input),
        "DIM" => Some(Dim),
        "READ" => Some(Read),
        "LET" => Some(Let),
        "GOTO" => Some(Goto),
        "RUN" => Some(Run),
        "IF" => Some(If),
        "RESTORE" => Some(Restore),
        "GOSUB" => Some(Gosub),
        "RETURN" => Some(Return),
        "REM" => Some(Rem),
        "STOP" => Some(Stop),
        "ON" => Some(On),
        "PRINT" | "?" => Some(Print),
        "THEN" => Some(Then),
        "TO" => Some(To),
        _ => None,
    }
}

