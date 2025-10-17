mod errors;
mod lexer;
mod parser;
mod program;
mod runtime;
mod statements;
mod tokens;
mod value;

use anyhow::Result;
use std::io::{self, Write};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Notes:
/// - Goal: Recreate 6502 BASIC semantics/behavior in Rust.
/// - Layout: modules for lexing, parsing/eval, program storage (line/statement),
///   runtime (variables/arrays/strings), and statements.
fn main() -> Result<()> {
    let mut vm = runtime::Vm::new();

    // switch off debug mode
    vm.debug = false;

    // Ctrl-C handling: convert to STOP/CONT semantics in run loop
    let interrupted = Arc::new(AtomicBool::new(false));
    vm.set_interrupt_flag(interrupted.clone());
    {
        let flag = interrupted.clone();
        ctrlc::set_handler(move || {
            flag.store(true, Ordering::SeqCst);
        })?;
    }

    println!("M6502 BASIC (Rust) — initial REPL; type HELP for help");

    loop {
        // If running and Ctrl-C pressed: simulate STOP (BREAK) and allow CONT
        if interrupted.swap(false, Ordering::SeqCst) {
            if let Some(cl) = vm.current_line {
                vm.log_debug(format!("[REPL] Ctrl-C at line {}", cl));
                eprintln!("?BREAK IN {}", cl);
                if let Some(nl) = vm.next_line_after(cl) {
                    vm.jump_to = Some(nl);
                }
                vm.halted = true;
                continue;
            } else {
                vm.log_debug("[REPL] Ctrl-C at READY");
                println!("^C");
                continue;
            }
        }

        // Prompt
        print!("READY. ");
        io::stdout().flush()?;

        // Read a line
        let mut line = String::new();
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            vm.log_debug("[REPL] EOF (Ctrl-D)");
            println!("BYE");
            break;
        }
        let line = line.trim_end_matches(['\n', '\r']).to_string();

        if let Err(e) = handle_line(&mut vm, &line) {
            eprintln!("?{}", e);
        }
    }

    Ok(())
}

fn handle_line(vm: &mut runtime::Vm, src: &str) -> Result<()> {
    // Supported input forms:
    //  1) empty line: ignore
    //  2) starts with digits: program line (insert into Program)
    //  3) immediate statement: execute now (PRINT, LET, ...)
    let s = src.trim();
    if s.is_empty() {
        return Ok(());
    }

    if let Some((first_no, rest)) = program::parse_leading_line_number(s) {
        // Support multiple line entries in one input: 10 ... :20 ... :30 ...
        // Split by ':' then treat a chunk starting with digits as a new line.
        let mut parts: Vec<&str> = Vec::new();
        let mut start = 0usize;
        let bytes = rest.as_bytes();
        for i in 0..bytes.len() {
            if bytes[i] as char == ':' {
                parts.push(&rest[start..i]);
                start = i + 1;
            }
        }
        parts.push(&rest[start..]);

        // First chunk belongs to first_no
        let first_chunk = parts.get(0).map(|s| s.trim()).unwrap_or("");
        if first_chunk.is_empty() {
            vm.program.delete_line(first_no);
        } else {
            let tokens = lexer::crunch(first_chunk);
            vm.program.insert_line(first_no, tokens);
        }

        // Subsequent chunks like "20 READ A" → parse leading line number
        for chunk in parts.into_iter().skip(1) {
            let c = chunk.trim_start();
            if c.is_empty() {
                continue;
            }
            if let Some((ln, stmt)) = program::parse_leading_line_number(c) {
                if stmt.trim().is_empty() {
                    vm.program.delete_line(ln);
                } else {
                    let toks = lexer::crunch(stmt);
                    vm.program.insert_line(ln, toks);
                }
            } else {
                // If no leading number after ':', treat as same line's additional statements
                // Append to the previous line by concatenating tokens with ':'
                // Simpler: insert as part of first_no line tail
                let mut toks = lexer::crunch(":");
                let mut more = lexer::crunch(c);
                toks.append(&mut more);
                // Re-read existing tokens and append
                if let Some(pl) = vm.program.lines.get(&first_no).cloned() {
                    let mut combined = pl.tokens;
                    combined.append(&mut toks);
                    vm.program.insert_line(first_no, combined);
                }
            }
        }
        return Ok(());
    }

    // Immediate statement: parse and execute.
    let tokens = lexer::crunch(s);
    statements::execute_direct(vm, &tokens)
}
