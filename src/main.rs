mod tokens;
mod value;
mod errors;
mod lexer;
mod parser;
mod program;
mod runtime;
mod statements;

use anyhow::Result;
use rustyline::{error::ReadlineError, DefaultEditor};

/// Notes:
/// - Goal: Recreate 6502 BASIC semantics/behavior in Rust.
/// - Layout: modules for lexing, parsing/eval, program storage (line/statement),
///   runtime (variables/arrays/strings), and statements.
/// - First cut: minimal REPL with LET, PRINT, basic expressions and string concat.
fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut vm = runtime::Vm::new();

    println!("M6502 BASIC (Rust) — initial REPL; type HELP for help");

    loop {
        match rl.readline("READY. ") {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).ok();
                if let Err(e) = handle_line(&mut vm, &line) {
                    eprintln!("?{}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
            }
            Err(ReadlineError::Eof) => {
                println!("BYE");
                break;
            }
            Err(e) => {
                eprintln!("I/O error: {e}");
            }
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

    if let Some((line_no, rest)) = program::parse_leading_line_number(s) {
        if rest.trim().is_empty() {
            // Empty content with a line number => delete that line.
            vm.program.delete_line(line_no);
            return Ok(());
        }
        let tokens = lexer::crunch(rest);
        vm.program.insert_line(line_no, tokens);
        return Ok(());
    }

    // Immediate statement: parse and execute.
    let tokens = lexer::crunch(s);
    statements::execute_direct(vm, &tokens)
}
