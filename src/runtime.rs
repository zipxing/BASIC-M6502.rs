use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::fs::OpenOptions;
use std::io::Write as IoWrite;
use crate::program::Program;
use crate::value::Value;
use crate::tokens::{Tok, TokenKind};
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
    // DATA/READ cursor
    pub data_line_pos: usize,
    pub data_tok_pos: Option<usize>,
    // Pseudo RNG state
    pub rng_seed: u64,
    pub debug: bool,
    pub interrupt_flag: Option<Arc<AtomicBool>>,
}

impl Vm {
    pub fn new() -> Self { Self { program: Program::default(), vars: HashMap::new(), halted: false, jump_to: None, gosub_stack: Vec::new(), for_stack: Vec::new(), current_line: None, line_order: Vec::new(), data_line_pos: 0, data_tok_pos: None, rng_seed: 0x1234_5678_9abc_def0, debug: true, interrupt_flag: None } }

    /// Prepare a fresh run: clear variables and reset DATA pointer.
    pub fn prepare_full_run(&mut self) {
        self.vars.clear();
        self.restore_data(None);
        self.jump_to = None;
    }

    /// Run the current program (respects existing jump_to/current_line for CONT).
    pub fn run(&mut self) {
        let lines: Vec<u16> = self.program.lines.keys().cloned().collect();
        let mut i = 0usize;
        self.halted = false;
        self.line_order = lines.clone();
        self.gosub_stack.clear();
        self.for_stack.clear();
        // DATA cursor unchanged here; RUN command should call prepare_full_run() beforehand
        // If a resume target exists (e.g., CONT after STOP), start there
        if let Some(dst) = self.jump_to.take() {
            if let Some(pos) = lines.iter().position(|x| *x == dst) { i = pos; }
        }
        while i < lines.len() {
            let ln = lines[i];
            self.current_line = Some(ln);
            // Poll Ctrl-C interrupt flag while running
            if let Some(flag) = &self.interrupt_flag {
                if flag.swap(false, Ordering::SeqCst) {
                    self.log_debug(format!("[RUN] Ctrl-C at line {}", ln));
                    eprintln!("?BREAK IN {}", ln);
                    if let Some(nl) = self.next_line_after(ln) { self.jump_to = Some(nl); }
                    self.halted = true;
                }
            }
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
                // otherwise execute as immediate; propagate errors as halts with line number
                match execute_direct(self, &s) {
                    Ok(()) => {}
                    Err(e) => {
                        if let Some(cl) = self.current_line { eprintln!("?{} IN {}", e, cl); } else { eprintln!("?{}", e); }
                        self.halted = true;
                    }
                }
                if self.halted { break; }
                // Also allow breaking mid-line if interrupt was raised
                if let Some(flag) = &self.interrupt_flag { if flag.load(Ordering::SeqCst) { break; } }
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
        // Preserve current_line on halt (e.g., STOP) to allow CONT to resume
        if !self.halted {
            self.current_line = None;
        }
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

impl Vm {
    pub fn restore_data(&mut self, at_line: Option<u16>) {
        if let Some(ln) = at_line {
            if let Some(pos) = self.line_order.iter().position(|x| *x == ln) {
                self.data_line_pos = pos;
            } else {
                self.data_line_pos = 0;
            }
        } else {
            self.data_line_pos = 0;
        }
        self.data_tok_pos = None;
    }

    /// Fetch next DATA value scanning program lines. Returns None if no more.
    pub fn next_data_value(&mut self) -> Option<Value> {
        while self.data_line_pos < self.line_order.len() {
            let ln = self.line_order[self.data_line_pos];
            let pl = self.program.lines.get(&ln)?;
            let mut pos = match self.data_tok_pos {
                Some(p) => p,
                None => {
                    // find DATA token
                    let mut idx = None;
                    for (i, t) in pl.tokens.iter().enumerate() {
                        if let Tok::Keyword(TokenKind::Data) = t { idx = Some(i+1); break; }
                    }
                    match idx { Some(p) => p, None => { self.data_line_pos += 1; continue; } }
                }
            };
            // skip commas
            while pos < pl.tokens.len() { if matches!(pl.tokens[pos], Tok::Symbol(',')) { pos+=1; } else { break; } }
            if pos >= pl.tokens.len() { self.data_line_pos += 1; self.data_tok_pos = None; continue; }
            match &pl.tokens[pos] {
                Tok::Number(n) => { self.data_tok_pos = Some(pos+1); return Some(Value::Number(*n)); }
                Tok::String(s) => { self.data_tok_pos = Some(pos+1); return Some(Value::Str(s.clone())); }
                // end of DATA on this line (e.g., colon or not a literal)
                Tok::Symbol(':') => { self.data_line_pos += 1; self.data_tok_pos = None; continue; }
                _ => { self.data_line_pos += 1; self.data_tok_pos = None; continue; }
            }
        }
        None
    }

    /// Simple LCG RNG producing [0,1) doubles; not hardware-accurate, but sufficient for RND.
    pub fn next_rand(&mut self) -> f64 {
        // Numerical Recipes LCG parameters
        self.rng_seed = self.rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let v = (self.rng_seed >> 11) as f64 / ((1u64 << 53) as f64);
        if v >= 1.0 { 0.999999999999 } else { v }
    }

    /// Append debug line to debug.log if debug is enabled.
    pub fn log_debug<S: AsRef<str>>(&self, s: S) {
        if !self.debug { return; }
        if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("debug.log") {
            let _ = writeln!(f, "{}", s.as_ref());
        }
    }

    pub fn set_interrupt_flag(&mut self, flag: Arc<AtomicBool>) {
        self.interrupt_flag = Some(flag);
    }
}

