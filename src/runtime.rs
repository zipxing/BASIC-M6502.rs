use std::collections::HashMap;
use crate::program::Program;
use crate::value::Value;
use crate::tokens::{Tok, TokenKind};
use crate::lexer::crunch;
use crate::statements::execute_direct;

/// Virtual machine state: holds program and variables.
#[derive(Default)]
pub struct Vm {
    pub program: Program,
    pub vars: HashMap<String, Value>,
    pub halted: bool,
    pub jump_to: Option<u16>,
    pub gosub_stack: Vec<GosubFrame>,
    pub for_stack: Vec<ForFrame>,
    pub current_line: Option<u16>,
    pub line_order: Vec<u16>,
}

impl Vm {
    pub fn new() -> Self { Self { program: Program::default(), vars: HashMap::new(), halted: false, jump_to: None, gosub_stack: Vec::new(), for_stack: Vec::new(), current_line: None, line_order: Vec::new() } }

    /// Run the current program from the lowest line.
    pub fn run(&mut self) {
        let lines: Vec<u16> = self.program.lines.keys().cloned().collect();
        let mut i = 0usize;
        self.halted = false;
        self.line_order = lines.clone();
        self.gosub_stack.clear();
        self.for_stack.clear();
        while i < lines.len() {
            let ln = lines[i];
            self.current_line = Some(ln);
            let Some(pl) = self.program.lines.get(&ln) else { i += 1; continue };
            // Split by ':' for multiple statements per line (simple pass over symbols)
            let mut stmt: Vec<Tok> = Vec::new();
            let mut stmts: Vec<Vec<Tok>> = Vec::new();
            for t in &pl.tokens {
                if matches!(t, Tok::Symbol(':')) {
                    if !stmt.is_empty() { stmts.push(std::mem::take(&mut stmt)); }
                } else {
                    stmt.push(t.clone());
                }
            }
            if !stmt.is_empty() { stmts.push(stmt); }

            for s in stmts {
                if s.is_empty() { continue; }
                // handle minimal GOTO inline: [GOTO] <number>
                if let Some(Tok::Keyword(TokenKind::Goto)) = s.get(0) {
                    if let Some(Tok::Number(n)) = s.get(1) {
                        self.jump_to = Some((*n as i64).clamp(0, u16::MAX as i64) as u16);
                        break;
                    }
                }
                // otherwise execute as immediate
                let _ = execute_direct(self, &s);
                if self.halted { break; }
                if self.jump_to.is_some() { break; }
            }

            if self.halted { break; }
            if let Some(dst) = self.jump_to.take() {
                // find index of destination line (if exists), else stop
                if let Some(pos) = lines.iter().position(|x| *x == dst) { i = pos; } else { break; }
            } else {
                i += 1;
            }
        }
        self.current_line = None;
    }

    pub fn next_line_after(&self, ln: u16) -> Option<u16> {
        if let Some(pos) = self.line_order.iter().position(|x| *x == ln) {
            self.line_order.get(pos + 1).cloned()
        } else { None }
    }
}

#[derive(Debug, Clone)]
pub struct GosubFrame {
    pub return_line: u16,
}

#[derive(Debug, Clone)]
pub struct ForFrame {
    pub var: String,
    pub end: f64,
    pub step: f64,
    pub start_line: u16,
}

