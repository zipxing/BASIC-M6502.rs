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
    New,
    Clear,
    Save,
    Load,
    Cont,
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
        "NEW" => Some(New),
        "CLEAR" => Some(Clear),
        "SAVE" => Some(Save),
        "LOAD" => Some(Load),
        "CONT" => Some(Cont),
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

/// Map a keyword token to its canonical BASIC name for listing.
pub fn keyword_name(k: TokenKind) -> &'static str {
    use TokenKind::*;
    match k {
        End => "END",
        For => "FOR",
        Next => "NEXT",
        Data => "DATA",
        Input => "INPUT",
        Dim => "DIM",
        Read => "READ",
        Let => "LET",
        Goto => "GOTO",
        Run => "RUN",
        List => "LIST",
        New => "NEW",
        Clear => "CLEAR",
        Save => "SAVE",
        Load => "LOAD",
        Cont => "CONT",
        If => "IF",
        Restore => "RESTORE",
        Gosub => "GOSUB",
        Return => "RETURN",
        Rem => "REM",
        Stop => "STOP",
        On => "ON",
        Print => "PRINT",
        Then => "THEN",
        To => "TO",
    }
}

