mod errors;
mod lexer;
mod parser;
mod program;
mod runtime;
mod statements;
mod tokens;
mod value;

use anyhow::Result;
use std::io::{self, BufRead, IsTerminal};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use rustyline::{Editor, error::ReadlineError, completion::{Completer, Pair}, Context, Helper};
use rustyline::validate::{Validator, ValidationResult, ValidationContext};
use rustyline::highlight::{Highlighter, CmdKind};
use rustyline::hint::Hinter;
use std::borrow::Cow;

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

    // Check if running in interactive mode (terminal) or batch mode (pipe/file)
    if std::io::stdin().is_terminal() {
        run_interactive_mode(&mut vm, interrupted)?;
    } else {
        run_batch_mode(&mut vm)?;
    }

    Ok(())
}

/// Interactive mode with rustyline (line editing, history, completion)
fn run_interactive_mode(vm: &mut runtime::Vm, interrupted: Arc<AtomicBool>) -> Result<()> {
    let mut rl = Editor::<BasicHelper, _>::new()?;
    
    // Set up helper with command completion
    rl.set_helper(Some(BasicHelper));
    
    // Load history from file
    let history_file = ".basic_history";
    if rl.load_history(history_file).is_err() {
        // First run, no history file yet
    }

    println!("M6502 BASIC (Rust) — interactive REPL; type HELP for help");
    println!("Features: Command history (↑/↓), line editing, Tab completion");

    loop {
        // Check for Ctrl-C during program execution
        if interrupted.swap(false, Ordering::SeqCst) {
            if let Some(cl) = vm.current_line {
                vm.log_debug(format!("[REPL] Ctrl-C at line {}", cl));
                eprintln!("?BREAK IN {}", cl);
                if let Some(nl) = vm.next_line_after(cl) {
                    vm.jump_to = Some(nl);
                }
                vm.halted = true;
                continue;
            }
            // Ctrl-C at READY prompt is handled by rustyline
        }

        match rl.readline("READY. ") {
            Ok(line) => {
                // Add to history (rustyline will deduplicate automatically)
                if !line.trim().is_empty() {
                    rl.add_history_entry(&line)?;
                }
                
                if let Err(e) = handle_line(vm, &line) {
                    eprintln!("?{}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl-C at READY prompt
                vm.log_debug("[REPL] Ctrl-C at READY");
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                // Ctrl-D to exit
                vm.log_debug("[REPL] EOF (Ctrl-D)");
                println!("BYE");
                break;
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }
    }

    // Save history before exit
    if let Err(e) = rl.save_history(history_file) {
        eprintln!("Warning: Could not save history: {}", e);
    }

    Ok(())
}

/// Batch mode using standard stdin (for pipes and file redirection)
fn run_batch_mode(vm: &mut runtime::Vm) -> Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line?;
        if let Err(e) = handle_line(vm, &line) {
            eprintln!("?{}", e);
        }
    }

    Ok(())
}

/// Helper for rustyline with command completion
struct BasicHelper;

impl Completer for BasicHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        // BASIC keywords and commands for completion
        let keywords = vec![
            // Statements
            "PRINT", "LET", "INPUT", "IF", "THEN", "GOTO", "GOSUB", "RETURN",
            "FOR", "NEXT", "STEP", "TO", "END", "STOP", "CONT",
            "DATA", "READ", "RESTORE", "DIM", "REM",
            "ON", "SAVE", "LOAD",
            // Commands
            "RUN", "LIST", "NEW", "CLEAR", "HELP",
            // Functions
            "ABS", "INT", "SGN", "SQR", "SIN", "COS", "TAN", "ATN",
            "EXP", "LOG", "RND",
            "LEN", "LEFT$", "RIGHT$", "MID$", "CHR$", "ASC",
            "VAL", "STR$", "SPACE$", "INSTR",
        ];

        // Find the start of the current word
        let start = line[..pos]
            .rfind(|c: char| c.is_whitespace() || c == '(' || c == ',')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        let prefix = line[start..pos].to_uppercase();
        
        if prefix.is_empty() {
            return Ok((start, vec![]));
        }

        // Filter keywords that start with the prefix
        let matches: Vec<Pair> = keywords
            .iter()
            .filter(|kw| kw.starts_with(&prefix))
            .map(|kw| Pair {
                display: kw.to_string(),
                replacement: kw.to_string(),
            })
            .collect();

        Ok((start, matches))
    }
}

impl Validator for BasicHelper {
    fn validate(&self, _ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

impl Highlighter for BasicHelper {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> Cow<'l, str> {
        Cow::Borrowed(line)
    }

    fn highlight_char(&self, _line: &str, _pos: usize, _forced: CmdKind) -> bool {
        false
    }
}

impl Hinter for BasicHelper {
    type Hint = String;

    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        None
    }
}

impl Helper for BasicHelper {}

fn handle_line(vm: &mut runtime::Vm, src: &str) -> Result<()> {
    // Supported input forms:
    //  1) empty line: ignore
    //  2) starts with digits: program line (insert into Program)
    //  3) immediate statement: execute now (PRINT, LET, ...)
    let s = src.trim();
    if s.is_empty() {
        return Ok(());
    }

    // Special handling for HELP command
    if s.to_uppercase() == "HELP" {
        print_help();
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

fn print_help() {
    println!("M6502 BASIC Commands:");
    println!();
    println!("Program Control:");
    println!("  RUN            - Execute the program");
    println!("  LIST           - List program lines");
    println!("  NEW            - Clear program and variables");
    println!("  CLEAR          - Clear variables only");
    println!("  CONT           - Continue after STOP");
    println!();
    println!("Statements:");
    println!("  PRINT / ?      - Output text and values");
    println!("  LET var=expr   - Assign value to variable");
    println!("  INPUT \"prompt\"; var - Read user input");
    println!("  IF cond THEN   - Conditional execution");
    println!("  GOTO line      - Jump to line number");
    println!("  GOSUB line     - Call subroutine");
    println!("  RETURN         - Return from subroutine");
    println!("  FOR..NEXT      - Loop with counter");
    println!("  END            - End program");
    println!("  STOP           - Pause program (use CONT to resume)");
    println!();
    println!("Data:");
    println!("  DATA           - Define data values");
    println!("  READ           - Read data into variables");
    println!("  RESTORE        - Reset data pointer");
    println!("  DIM array(n)   - Declare array");
    println!();
    println!("Functions:");
    println!("  Math: ABS INT SGN SQR SIN COS TAN ATN EXP LOG RND");
    println!("  String: LEN LEFT$ RIGHT$ MID$ CHR$ ASC VAL STR$ SPACE$ INSTR");
    println!();
    println!("Tips:");
    println!("  - Use ↑/↓ arrows to navigate command history");
    println!("  - Press Tab for keyword completion");
    println!("  - String variables end with $ (e.g., NAME$)");
    println!("  - Ctrl-C during RUN to break, then CONT to continue");
    println!("  - Ctrl-D or Ctrl-C at prompt to exit");
}
