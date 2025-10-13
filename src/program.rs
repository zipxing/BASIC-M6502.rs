use crate::lexer::take_leading_line_number;
use crate::tokens::Tok;
use std::collections::BTreeMap;

/// Program line structure, conceptually like [line_no][text]\0.
#[derive(Debug, Clone)]
pub struct ProgramLine {
    pub line_no: u16,
    pub tokens: Vec<Tok>,
}

#[derive(Default, Debug)]
pub struct Program {
    pub lines: BTreeMap<u16, ProgramLine>,
}

impl Program {
    pub fn insert_line(&mut self, line_no: u16, tokens: Vec<Tok>) {
        if tokens.is_empty() {
            self.lines.remove(&line_no);
        } else {
            self.lines.insert(line_no, ProgramLine { line_no, tokens });
        }
    }
    pub fn delete_line(&mut self, line_no: u16) { self.lines.remove(&line_no); }
}

/// Parse an optional leading line number (wrapper for lexer helper).
pub fn parse_leading_line_number(src: &str) -> Option<(u16, &str)> {
    take_leading_line_number(src)
}

