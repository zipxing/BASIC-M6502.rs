use anyhow::{bail, Result};
use crate::runtime::Vm;
use crate::tokens::{Tok, TokenKind};
use crate::parser::{Cursor, parse_expression_with_vm};
use crate::value::Value;

/// Execute immediate statements (no line number).
/// Supports: PRINT, LET, and implicit assignment (IDENT = expr).
pub fn execute_direct(vm: &mut Vm, toks: &[Tok]) -> Result<()> {
    // Support colon-separated immediate statements as well.
    // Split on ':' and execute sequentially.
    let mut parts: Vec<&[Tok]> = Vec::new();
    let mut start = 0usize;
    for (idx, t) in toks.iter().enumerate() {
        if matches!(t, Tok::Symbol(':')) {
            if idx > start { parts.push(&toks[start..idx]); }
            start = idx + 1;
        }
    }
    if start < toks.len() { parts.push(&toks[start..]); }

    for part in parts {
        let mut cur = Cursor::new(part);
        // Fallback: treat IDENT matching command names as keywords (robust with custom lexers)
        if let Some(Tok::Ident(name)) = cur.peek() {
            let up = name.to_ascii_uppercase();
            match up.as_str() {
                "PRINT" => { cur.next(); exec_print(vm, &mut cur)?; continue; }
                "IF" => { cur.next(); exec_if(vm, &mut cur)?; continue; }
                "RUN" => { cur.next(); vm.run(); continue; }
                "LIST" => { cur.next(); vm.program.list(); continue; }
                "CLEAR" => { cur.next(); vm.vars.clear(); println!("READY."); continue; }
                "NEW" => { cur.next(); vm.vars.clear(); vm.program.clear(); println!("READY."); continue; }
                "GOSUB" => { cur.next(); exec_gosub(vm, &mut cur)?; continue; }
                "RETURN" => { cur.next(); exec_return(vm)?; continue; }
                "FOR" => { cur.next(); exec_for(vm, &mut cur)?; continue; }
                "NEXT" => { cur.next(); exec_next(vm, &mut cur)?; continue; }
                _ => {}
            }
        }
        match cur.peek() {
            Some(Tok::Keyword(TokenKind::Print)) => exec_print(vm, &mut cur),
            Some(Tok::Keyword(TokenKind::If)) => { cur.next(); exec_if(vm, &mut cur) }
            Some(Tok::Keyword(TokenKind::Gosub)) => { cur.next(); exec_gosub(vm, &mut cur) }
            Some(Tok::Keyword(TokenKind::Return)) => { cur.next(); exec_return(vm) }
            Some(Tok::Keyword(TokenKind::For)) => { cur.next(); exec_for(vm, &mut cur) }
            Some(Tok::Keyword(TokenKind::Next)) => { cur.next(); exec_next(vm, &mut cur) }
        Some(Tok::Keyword(TokenKind::Let)) => { cur.next(); exec_assignment(vm, &mut cur) },
        Some(Tok::Ident(_)) => exec_assignment(vm, &mut cur),
        Some(Tok::Keyword(TokenKind::Run)) => { cur.next(); vm.run(); Ok(()) }
        Some(Tok::Keyword(TokenKind::List)) => { cur.next(); vm.program.list(); Ok(()) }
        Some(Tok::Keyword(TokenKind::Clear)) => { cur.next(); vm.vars.clear(); println!("READY."); Ok(()) }
        Some(Tok::Keyword(TokenKind::New)) => { cur.next(); vm.vars.clear(); vm.program.clear(); println!("READY."); Ok(()) }
        Some(Tok::Keyword(TokenKind::Dim)) => { bail!("DIM not implemented") }
        _ => bail!("SYNTAX ERROR"),
        }?;
    }
    Ok(())
}

fn exec_print(vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    // Skip PRINT token or '?' symbol
    match cur.peek() { Some(Tok::Keyword(TokenKind::Print)) => { cur.next(); }, _ => {} }
    // Simplified: read expressions to end of line, separated by comma/semicolon.
    let mut first = true;
    loop {
        if let Some(val) = parse_expression_with_vm(cur, vm) {
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
    let val = parse_expression_with_vm(cur, vm).ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))?;
    vm.vars.insert(name, val);
    Ok(())
}

fn exec_if(vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    // Minimal: IF <expr> THEN <line>|<immediate statements>
    let cond = parse_expression_with_vm(cur, vm).ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))?;
    let is_then = match cur.next() {
        Some(Tok::Keyword(TokenKind::Then)) => true,
        Some(Tok::Ident(s)) if s.to_ascii_uppercase()=="THEN" => true,
        _ => false,
    };
    if !is_then { bail!("SYNTAX ERROR"); }
    // Next token: number => branch to line, else treat remainder as immediate
    match cur.peek() {
        Some(Tok::Number(n)) => {
            if cond.as_number() != 0.0 {
                vm.jump_to = Some((*n as i64).clamp(0, u16::MAX as i64) as u16);
            }
            Ok(())
        }
        _ => {
            if cond.as_number() != 0.0 {
                // Execute remaining tokens as immediate statement
                let rest = &cur.toks[cur.i..];
                let _ = execute_direct(vm, rest);
            }
            Ok(())
        }
    }
}

fn exec_gosub(vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    let Tok::Number(n) = cur.next().ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))? else { bail!("SYNTAX ERROR") };
    let ret = vm.current_line.and_then(|ln| vm.next_line_after(ln)).ok_or_else(|| anyhow::anyhow!("RETURN WITHOUT GOSUB"))?;
    vm.gosub_stack.push(crate::runtime::GosubFrame { return_line: ret });
    vm.jump_to = Some((*n as i64).clamp(0, u16::MAX as i64) as u16);
    Ok(())
}

fn exec_return(vm: &mut Vm) -> Result<()> {
    let frame = vm.gosub_stack.pop().ok_or_else(|| anyhow::anyhow!("RETURN WITHOUT GOSUB"))?;
    vm.jump_to = Some(frame.return_line);
    Ok(())
}

fn exec_for(vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    // FOR I=1 TO 10 STEP 2
    let var = match cur.next() { Some(Tok::Ident(s)) => s.clone(), _ => bail!("SYNTAX ERROR") };
    match cur.next() { Some(Tok::Symbol('=')) => {}, _ => bail!("SYNTAX ERROR") }
    let start = parse_expression_with_vm(cur, vm).ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))?.as_number();
    let is_to = match cur.next() {
        Some(Tok::Keyword(TokenKind::To)) => true,
        Some(Tok::Ident(s)) if s.to_ascii_uppercase()=="TO" => true,
        _ => false,
    };
    if !is_to { bail!("SYNTAX ERROR"); }
    let end = parse_expression_with_vm(cur, vm).ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))?.as_number();
    let mut step = 1.0;
    if let Some(tok) = cur.peek() {
        match tok {
            Tok::Ident(s) if s.to_ascii_uppercase()=="STEP" => { cur.next(); step = parse_expression_with_vm(cur, vm).ok_or_else(|| anyhow::anyhow!("SYNTAX ERROR"))?.as_number(); }
            _ => {}
        }
    }
    vm.vars.insert(var.clone(), Value::Number(start));
    let line = vm.current_line.ok_or_else(|| anyhow::anyhow!("FOR without line context"))?;
    vm.for_stack.push(crate::runtime::ForFrame { var, end, step, start_line: line });
    Ok(())
}

fn exec_next(vm: &mut Vm, cur: &mut Cursor) -> Result<()> {
    // NEXT I  (variable optional; if present must match top of stack)
    let name_opt = match cur.peek() { Some(Tok::Ident(s)) => Some(s.clone()), _ => None };
    if name_opt.is_some() { cur.next(); }
    let frame = match vm.for_stack.last_mut() { Some(f) => f, None => bail!("NEXT WITHOUT FOR") };
    if let Some(nm) = &name_opt { if *nm != frame.var { bail!("NEXT WITHOUT FOR") } }
    let curv = vm.vars.get(&frame.var).and_then(|v| if let Value::Number(n)=v { Some(*n) } else { None }).unwrap_or(0.0);
    let newv = curv + frame.step;
    vm.vars.insert(frame.var.clone(), Value::Number(newv));
    let continue_loop = if frame.step >= 0.0 { newv <= frame.end } else { newv >= frame.end };
    if continue_loop {
        vm.jump_to = Some(frame.start_line);
    } else {
        vm.for_stack.pop();
    }
    Ok(())
}

