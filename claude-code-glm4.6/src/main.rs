use std::io::{self, Write};
use std::env;
use std::fs;

mod lexer;
mod parser;
mod evaluator;
mod runtime;
mod statements;
mod functions;
mod error;
mod utils;

use runtime::memory::MemoryManager;
use lexer::{Lexer, Token};
use evaluator::ExpressionEvaluator;
use statements::StatementExecutor;
use error::{BasicError, BasicResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Microsoft BASIC 6502 - Rust Implementation v0.1.0");
    println!("Type 'HELP' for assistance, 'QUIT' to exit");
    println!();

    let mut mem = MemoryManager::new();
    let mut lexer = Lexer::new();
    let mut evaluator = ExpressionEvaluator::new();
    let mut executor = StatementExecutor::new();

    // Check if a file argument was provided
    let args: Vec<String> = env::args().collect();
    let file_mode = args.len() > 1;
    if file_mode {
        // Load and execute the file
        let filename = &args[1];
        let program = fs::read_to_string(filename)?;
        println!("Loading program from: {}", filename);
        println!();

        // Parse and store all lines from the file
        for line in program.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("'") {
                continue; // Skip empty lines and comments
            }

            // Tokenize and store the line
            match lexer.tokenize(line) {
                Ok(tokens) => {
                    if let Some(line_num) = extract_line_number(&tokens) {
                        // Check if this is a DATA statement - if so, process it immediately
                        if !tokens.is_empty() && matches!(tokens[1], Token::Data) {
                            // Execute DATA statement to populate data values
                            if let Err(e) = executor.execute_statement(&tokens[1..], &mut mem, &mut evaluator) {
                                println!("Error in DATA statement '{}': {:?}", line, e);
                            }
                        }
                        // Store the line regardless of type (DATA lines are also stored for LIST command)
                        mem.store_line(line_num, tokens)?;
                    } else {
                        println!("Warning: Line without line number will be ignored: {}", line);
                    }
                }
                Err(e) => {
                    println!("Error in line '{}': {:?}", line, e);
                }
            }
        }

        println!("Program loaded. Type 'RUN' to execute.");
        println!();

        // In file mode, automatically run the program and then exit
        println!("Running program...");
        if let Err(e) = run_program(&mut mem, &mut executor, &mut evaluator) {
            eprintln!("ERROR: {}", e);
        }

        // Exit after file execution
        println!("Program execution completed. Exiting.");
        return Ok(());
    }

    loop {
        print!("READY.");
        io::stdout().flush()?;

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.is_empty() {
                    continue;
                }

                match input.to_uppercase().as_str() {
                    "QUIT" | "EXIT" => break,
                    "HELP" => {
                        print_help();
                        continue;
                    }
                    "NEW" => {
                        mem.clear();
                        println!("PROGRAM CLEARED");
                        continue;
                    }
                    "LIST" => {
                        mem.list_program();
                        continue;
                    }
                    "RUN" => {
                        if let Err(e) = run_program(&mut mem, &mut executor, &mut evaluator) {
                            eprintln!("ERROR: {}", e);
                        }
                        continue;
                    }
                    _ => {}
                }

                // 词法分析
                let tokens = match lexer.tokenize(input) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        eprintln!("SYNTAX ERROR: {}", e);
                        continue;
                    }
                };

                // 检查是否有行号
                if let Some(line_number) = extract_line_number(&tokens) {
                    // Check if this is a DATA statement - if so, process it immediately
                    if tokens.len() > 1 && matches!(tokens[1], Token::Data) {
                        if let Err(e) = executor.execute_statement(&tokens[1..], &mut mem, &mut evaluator) {
                            eprintln!("ERROR: {}", e);
                        }
                    }
                    // 存储程序行
                    if let Err(e) = mem.store_line(line_number, tokens) {
                        eprintln!("ERROR: {}", e);
                    }
                } else {
                    // 立即执行
                    if let Err(e) = executor.execute_statement(&tokens, &mut mem, &mut evaluator) {
                        eprintln!("ERROR: {}", e);
                    }
                }
            }
            Err(e) => {
                // stdin读取失败，可能是因为在后台运行
                eprintln!("Input error: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn run_program(mem: &mut MemoryManager, executor: &mut StatementExecutor, evaluator: &mut ExpressionEvaluator) -> BasicResult<()> {
    // Get program lines in order
    let execution_order = mem.get_execution_order();

    if execution_order.is_empty() {
        println!("NO PROGRAM TO RUN");
        return Ok(());
    }

    // First, process all DATA statements to populate data values
    for &line_number in &execution_order {
        let line_tokens = if let Some(program_line) = mem.get_line(line_number) {
            program_line.tokens.clone()
        } else {
            continue;
        };

        // Skip the LineNumber token and check if it's a DATA statement
        if line_tokens.len() > 1 && matches!(line_tokens[1], Token::Data) {
            let tokens = &line_tokens[1..]; // Skip LineNumber
            if let Err(e) = executor.execute_statement(tokens, mem, evaluator) {
                eprintln!("ERROR IN DATA STATEMENT ON LINE {}: {}", line_number, e);
            }
        }
    }

    // Start execution from the first line
    let mut current_line_idx = 0;

    while current_line_idx < execution_order.len() {
        let line_number = execution_order[current_line_idx];
        mem.set_current_line(line_number);

        // Get a copy of the program line tokens to avoid borrowing issues
        let line_tokens = if let Some(program_line) = mem.get_line(line_number) {
            program_line.tokens.clone()
        } else {
            eprintln!("LINE {} NOT FOUND", line_number);
            break;
        };

        // Skip the first LineNumber token
        let tokens = &line_tokens[1..];

        if !tokens.is_empty() {
            // Skip DATA statements during normal execution (already processed)
            if matches!(tokens[0], Token::Data) {
                current_line_idx += 1;
                continue;
            }

            match executor.execute_statement(tokens, mem, evaluator) {
                Ok(_) => {
                    // Continue to next line
                    current_line_idx += 1;
                }
                Err(BasicError::GotoJump(jump_line)) => {
                    // Jump to different line
                    if let Some(jump_idx) = execution_order.iter().position(|&x| x == jump_line) {
                        current_line_idx = jump_idx;
                    } else {
                        eprintln!("LINE {} NOT FOUND", jump_line);
                        break;
                    }
                }
                Err(BasicError::GotoJumpWithStatement(jump_line, statement_idx)) => {
                    // Jump to specific line and statement (for single-line FOR loops)
                    // Use a loop to handle recursive GotoJumpWithStatement
                    let mut current_jump_line = jump_line;
                    let mut current_statement_idx = statement_idx;
                    
                    loop {
                        if let Some(jump_idx) = execution_order.iter().position(|&x| x == current_jump_line) {
                            current_line_idx = jump_idx;
                            mem.set_current_line(current_jump_line);
                            
                            // Get the line tokens
                            let jump_line_tokens = if let Some(program_line) = mem.get_line(current_jump_line) {
                                program_line.tokens.clone()
                            } else {
                                eprintln!("LINE {} NOT FOUND", current_jump_line);
                                break;
                            };
                            let jump_tokens = &jump_line_tokens[1..];
                            
                            // Execute from the specified statement index
                            match executor.execute_statement_from(jump_tokens, mem, evaluator, current_statement_idx) {
                                Ok(_) => {
                                    // Statement execution completed normally, continue to next line
                                    current_line_idx += 1;
                                    break; // Exit the inner loop, will continue with outer while loop
                                }
                                Err(BasicError::GotoJumpWithStatement(j, s)) => {
                                    // Another statement-level jump - continue the loop
                                    current_jump_line = j;
                                    current_statement_idx = s;
                                    continue;
                                }
                                Err(BasicError::GotoJump(j)) => {
                                    // Regular line jump
                                    if let Some(idx) = execution_order.iter().position(|&x| x == j) {
                                        current_line_idx = idx;
                                    } else {
                                        eprintln!("LINE {} NOT FOUND", j);
                                    }
                                    break;
                                }
                                Err(BasicError::GosubJump(j)) => {
                                    if let Some(idx) = execution_order.iter().position(|&x| x == j) {
                                        current_line_idx = idx;
                                    } else {
                                        eprintln!("LINE {} NOT FOUND", j);
                                    }
                                    break;
                                }
                                Err(BasicError::ReturnJump(j)) => {
                                    if let Some(idx) = execution_order.iter().position(|&x| x == j) {
                                        current_line_idx = idx + 1;
                                    } else {
                                        eprintln!("RETURN LINE {} NOT FOUND", j);
                                    }
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("ERROR ON LINE {}: {}", current_jump_line, e);
                                    break;
                                }
                            }
                        } else {
                            eprintln!("LINE {} NOT FOUND", current_jump_line);
                            break;
                        }
                    }
                }
                Err(BasicError::GosubJump(jump_line)) => {
                    // Jump to subroutine
                    if let Some(jump_idx) = execution_order.iter().position(|&x| x == jump_line) {
                        current_line_idx = jump_idx;
                    } else {
                        eprintln!("LINE {} NOT FOUND", jump_line);
                        break;
                    }
                }
                Err(BasicError::ReturnJump(return_line)) => {
                    // Return from subroutine - go to the line after the GOSUB call
                    if let Some(jump_idx) = execution_order.iter().position(|&x| x == return_line) {
                        current_line_idx = jump_idx + 1; // Go to the line after the GOSUB
                    } else {
                        eprintln!("RETURN LINE {} NOT FOUND", return_line);
                        break;
                    }
                }
                Err(BasicError::OutOfData) => {
                    // OUT OF DATA error should be displayed but execution continues
                    println!("OUT OF DATA");
                    current_line_idx += 1;
                }
                Err(e) => {
                    eprintln!("ERROR ON LINE {}: {}", line_number, e);
                    break;
                }
            }
        } else {
            current_line_idx += 1;
        }
    }

    Ok(())
}

fn extract_line_number(tokens: &[lexer::Token]) -> Option<u16> {
    if let Some(lexer::Token::LineNumber(num)) = tokens.first() {
        Some(*num)
    } else {
        None
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  HELP  - Show this help message");
    println!("  NEW   - Clear current program");
    println!("  LIST  - List program lines");
    println!("  RUN   - Execute the program");
    println!("  QUIT  - Exit the interpreter");
    println!();
    println!("Enter BASIC statements directly or with line numbers to store them.");
}