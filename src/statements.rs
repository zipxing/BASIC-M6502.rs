use anyhow::{bail, Result};
use crate::runtime::Vm;
use crate::tokens::{Tok, TokenKind};
use crate::parser::{Cursor, parse_expression};
use crate::value::Value;

/// Execute immediate statements (no line number).
/// Supports: PRINT, LET, and implicit assignment (IDENT = expr).
pub fn execute_direct(vm: &mut Vm, toks: &[Tok]) -> Result<()> {
    let mut cur = Cursor::new(toks);
    match cur.peek() {
        Some(Tok::Keyword(TokenKind::Print)) => exec_print(vm, &mut cur),
        Some(Tok::Keyword(TokenKind::Let)) => { cur.next(); exec_assignment(vm, &mut cur) },
        Some(Tok::Ident(_)) => exec_assignment(vm, &mut cur),
        Some(Tok::Keyword(TokenKind::Run)) => { println!("RUN (未实现)"); Ok(()) }
        Some(Tok::Keyword(TokenKind::List)) => { println!("LIST (未实现)"); Ok(()) }
        _ => bail!("SYNTAX ERROR"),
    }
}

fn exec_print(_vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    // Skip PRINT token or '?' symbol
    match cur.peek() { Some(Tok::Keyword(TokenKind::Print)) => { cur.next(); }, _ => {} }
    // Simplified: read expressions to end of line, separated by comma/semicolon.
    let mut first = true;
    loop {
        if let Some(val) = parse_expression(cur) {
            if !first { print!(" "); }
            match val { Value::Number(n) => print!("{}", n), Value::Str(s) => print!("{}", s) }
            first = false;
        }
        match cur.peek() {
            Some(Tok::Symbol(',')) | Some(Tok::Symbol(';')) => { cur.next(); continue; }
            _ => break,
        }
    }
    println!();
    Ok(())
}

fn exec_assignment(vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    let name = match cur.next() { Some(Tok::Ident(s)) => s.clone(), _ => bail!("SYNTAX ERROR") };
    match cur.next() { Some(Tok::Symbol('=')) => {}, _ => bail!("SYNTAX ERROR") }
    let val = parse_expression(cur).ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))?;
    vm.vars.insert(name, val);
    Ok(())
}

