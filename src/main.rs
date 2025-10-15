mod tokens;
mod value;
mod errors;
mod lexer;
mod parser;
mod program;
mod runtime;
mod statements;

use anyhow::Result;
use std::io::{self, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

/// Notes:
/// - Goal: Recreate 6502 BASIC semantics/behavior in Rust.
/// - Layout: modules for lexing, parsing/eval, program storage (line/statement),
///   runtime (variables/arrays/strings), and statements.
/// - First cut: minimal REPL with LET, PRINT, basic expressions and string concat.
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
                if let Some(nl) = vm.next_line_after(cl) { vm.jump_to = Some(nl); }
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
        let line = line.trim_end_matches(['\n','\r']).to_string();

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
        return Ok(())
    }

    if let Some((line_no, rest)) = program::parse_leading_line_number(s) {
        if rest.trim().is_empty() {
            // Empty content with a line number => delete that line.
            vm.program.delete_line(line_no);
            return Ok(())
        }
        let tokens = lexer::crunch(rest);
        vm.program.insert_line(line_no, tokens);
        return Ok(())
    }

    // Immediate statement: parse and execute.
    let tokens = lexer::crunch(s);
    statements::execute_direct(vm, &tokens)
}
