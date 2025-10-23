//! Statement executor for BASIC interpreter
//!
//! This module handles execution of BASIC statements.

use crate::error::{BasicError, BasicResult};
use crate::lexer::Token;
use crate::runtime::memory::{MemoryManager, Value, ForLoop};
use crate::evaluator::ExpressionEvaluator;

/// Helper function to extract line number from tokens
fn extract_line_number(tokens: &[Token]) -> Option<u16> {
    if let Some(Token::LineNumber(num)) = tokens.first() {
        Some(*num)
    } else {
        None
    }
}

pub struct StatementExecutor {
    // Execution state can be added here if needed
}

impl StatementExecutor {
    pub fn new() -> Self {
        Self {}
    }

    /// Execute a single statement
    pub fn execute_statement(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        if tokens.is_empty() {
            return Ok(true); // Empty line, continue execution
        }

        // Check the first token to determine statement type
        match &tokens[0] {
            Token::Let => self.execute_let(&tokens[1..], mem, evaluator),
            Token::Print => self.execute_print(&tokens[1..], mem, evaluator),
            Token::Input => self.execute_input(&tokens[1..], mem, evaluator),
            Token::Goto => self.execute_goto(&tokens[1..], mem),
            Token::Gosub => self.execute_gosub(&tokens[1..], mem),
            Token::Return => self.execute_return(mem),
            Token::If => self.execute_if(&tokens[1..], mem, evaluator),
            Token::For => self.execute_for(&tokens[1..], mem, evaluator),
            Token::Next => self.execute_next(&tokens[1..], mem, evaluator),
            Token::Data => self.execute_data(&tokens[1..], mem),
            Token::Read => self.execute_read(&tokens[1..], mem, evaluator),
            Token::Restore => self.execute_restore(mem),
            Token::Load => self.execute_load(&tokens[1..], mem),
            Token::Save => self.execute_save(&tokens[1..], mem),
            Token::End => Ok(false), // End program execution
            Token::Rem => Ok(true), // Remark - do nothing
            _ => {
                // If no statement keyword, try to evaluate as expression (implicitly LET)
                self.execute_implicit_let(tokens, mem, evaluator)
            }
        }
    }

    /// Execute LET statement: LET variable = expression
    fn execute_let(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        if tokens.len() < 3 {
            return Err(BasicError::Syntax);
        }

        // Check for assignment operator
        if !matches!(tokens[1], Token::Equal) {
            return Err(BasicError::Syntax);
        }

        // Get variable name
        let var_name = match &tokens[0] {
            Token::Identifier(name) => name,
            _ => return Err(BasicError::Syntax),
        };

        // Evaluate the expression on the right side
        let value = evaluator.evaluate(&tokens[2..], mem)?;

        // Store the variable
        mem.set_variable(var_name.clone(), value)?;

        Ok(true)
    }

    /// Execute implicit LET (assignment without LET keyword)
    fn execute_implicit_let(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        // Look for assignment operator
        let equal_pos = tokens.iter().position(|t| matches!(t, Token::Equal));

        if let Some(pos) = equal_pos {
            if pos == 0 || pos == tokens.len() - 1 {
                return Err(BasicError::Syntax);
            }

            // Get variable name
            let var_name = match &tokens[0] {
                Token::Identifier(name) => name,
                _ => return Err(BasicError::Syntax),
            };

            // Evaluate the expression after the equal sign
            let value = evaluator.evaluate(&tokens[pos + 1..], mem)?;

            // Store the variable
            mem.set_variable(var_name.clone(), value)?;

            Ok(true)
        } else {
            // If no assignment operator, just evaluate the expression
            evaluator.evaluate(tokens, mem)?;
            Ok(true)
        }
    }

    /// Execute PRINT statement
    fn execute_print(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        use std::io::{self, Write};

        if tokens.is_empty() {
            // PRINT with no arguments - just print newline
            println!();
            return Ok(true);
        }

        let mut i = 0;

        while i < tokens.len() {
            // Find the next separator or end of tokens
            let mut end = i;
            while end < tokens.len() {
                if matches!(tokens[end], Token::Comma | Token::Semicolon) {
                    break;
                }
                end += 1;
            }

            if i < end {
                // We have an expression to evaluate
                let expression = &tokens[i..end];

                // Evaluate the expression
                let value = evaluator.evaluate(expression, mem)?;

                // Check if current value was numeric before we potentially move it
                let is_numeric = matches!(value, crate::runtime::memory::Value::Float(_) | crate::runtime::memory::Value::Integer(_));

                // Print the value
                match value {
                    crate::runtime::memory::Value::String(s) => print!("{}", s),
                    crate::runtime::memory::Value::Float(f) => {
                        if f.fract() == 0.0 {
                            print!("{}.0", f);
                        } else {
                            print!("{}", f);
                        }
                    }
                    crate::runtime::memory::Value::Integer(n) => print!("{}", n),
                }

                // Check if there's a separator after this expression
                if end < tokens.len() {
                    match &tokens[end] {
                        Token::Comma => {
                            // Zone spacing (14 characters per zone in BASIC)
                            print!("{:<14}", "");
                        }
                        Token::Semicolon => {
                            // In BASIC, semicolon means no spacing at all between items
                            // Just move to the next item without adding any space
                        }
                        _ => {}
                    }
                    i = end + 1; // Skip the separator
                } else {
                    i = end; // End of tokens
                }
            } else {
                // No expression, just a separator or empty
                i += 1;
            }
        }

        // Add newline if needed (check if the last token was not a semicolon)
        if !tokens.is_empty() && matches!(tokens[tokens.len() - 1], Token::Semicolon) {
            // Statement ends with semicolon, no newline
            io::stdout().flush().unwrap_or(());
        } else {
            // Statement doesn't end with semicolon, add newline
            println!();
        }

        Ok(true)
    }

    /// Execute INPUT statement
    fn execute_input(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        use std::io::{self, Write};

        if tokens.is_empty() {
            return Err(BasicError::Syntax);
        }

        let mut i = 0;
        let mut prompt = String::new();
        let variable_name: String;

        // Check if first token is a string (prompt)
        if let Token::String(s) = &tokens[0] {
            prompt = s.clone();
            i += 1;

            // Check for separator after prompt
            if i >= tokens.len() {
                return Err(BasicError::Syntax);
            }

            match &tokens[i] {
                Token::Semicolon => {
                    // Semicolon means add "?" to prompt
                    prompt.push('?');
                    i += 1;
                }
                Token::Comma => {
                    // Comma means no additional prompt
                    i += 1;
                }
                _ => return Err(BasicError::Syntax),
            }
        } else {
            // No prompt, use default "?"
            prompt.push('?');
        }

        // Get variable name
        if i >= tokens.len() {
            return Err(BasicError::Syntax);
        }

        variable_name = match &tokens[i] {
            Token::Identifier(name) => name.clone(),
            _ => return Err(BasicError::Syntax),
        };

        // Display prompt
        print!("{}", prompt);
        io::stdout().flush().unwrap_or(());

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|_| BasicError::InputError("Failed to read input".to_string()))?;

        // Trim newline and convert to appropriate value
        let input = input.trim();
        let value = if variable_name.ends_with('$') {
            // String variable
            crate::runtime::memory::Value::String(input.to_string())
        } else {
            // Numeric variable - try to parse as number
            input.parse::<f64>()
                .map(crate::runtime::memory::Value::Float)
                .map_err(|_| BasicError::InputError("Invalid number".to_string()))?
        };

        // Store the variable
        mem.set_variable(variable_name, value)?;

        Ok(true)
    }

    /// Execute GOTO statement
    fn execute_goto(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
    ) -> BasicResult<bool> {
        if tokens.len() != 1 {
            return Err(BasicError::Syntax);
        }

        let line_number = match &tokens[0] {
            Token::Number(n) => *n as u16,
            _ => return Err(BasicError::Syntax),
        };

        // Check if the target line exists
        if !mem.program_lines().contains_key(&line_number) {
            return Err(BasicError::LineNumberNotFound(line_number));
        }

        // Set the current execution line
        mem.set_current_line(line_number);

        // Return false to stop normal sequential execution and jump to the target line
        Err(BasicError::GotoJump(line_number))
    }

    /// Execute GOSUB statement
    fn execute_gosub(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
    ) -> BasicResult<bool> {
        if tokens.len() != 1 {
            return Err(BasicError::Syntax);
        }

        let line_number = match &tokens[0] {
            Token::Number(n) => *n as u16,
            _ => return Err(BasicError::Syntax),
        };

        // Check if the target line exists
        if !mem.program_lines().contains_key(&line_number) {
            return Err(BasicError::LineNumberNotFound(line_number));
        }

        // Push return address onto the stack
        let current_line = mem.current_line();
        mem.push_gosub_return(current_line);

        // Set the current execution line
        mem.set_current_line(line_number);

        // Return false to stop normal sequential execution and jump to the subroutine
        Err(BasicError::GosubJump(line_number))
    }

    /// Execute RETURN statement
    fn execute_return(
        &mut self,
        mem: &mut MemoryManager,
    ) -> BasicResult<bool> {
        // Pop return address from stack
        let return_line = mem.pop_gosub_return()
            .ok_or(BasicError::ReturnWithoutGosub)?;

        // Set the current execution line to the return address
        mem.set_current_line(return_line);

        // Return false to stop normal sequential execution and jump back
        Err(BasicError::ReturnJump(return_line))
    }

    /// Execute IF statement
    fn execute_if(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        // IF syntax: IF condition THEN statement_or_line_number
        if tokens.is_empty() {
            return Err(BasicError::Syntax);
        }

        // Find the THEN token - everything before is the condition
        let mut then_pos = 0;
        while then_pos < tokens.len() && !matches!(tokens[then_pos], Token::Identifier(ref s) if s.to_uppercase() == "THEN") {
            then_pos += 1;
        }

        // Must have THEN
        if then_pos >= tokens.len() {
            return Err(BasicError::Syntax);
        }

        // Must have something after THEN
        if then_pos + 1 >= tokens.len() {
            return Err(BasicError::Syntax);
        }

        // Must have a condition (not empty before THEN)
        if then_pos == 0 {
            return Err(BasicError::Syntax);
        }

        // Evaluate the condition
        let condition_value = evaluator.evaluate(&tokens[..then_pos], mem)?;

        // Convert condition to boolean (0 = false, non-zero = true)
        let condition_is_true = match condition_value.to_float()? {
            f if f.abs() < f64::EPSILON => false,
            _ => true,
        };

        let then_tokens = &tokens[then_pos + 1..];

        if condition_is_true {
            // If condition is true, execute what's after THEN
            match then_tokens[0] {
                // Check if it's a line number (GOTO)
                Token::Number(n) => {
                    // IF condition THEN line_number - equivalent to GOTO
                    let line_number = n as u16;
                    return Err(BasicError::GotoJump(line_number));
                }
                Token::Identifier(ref s) if s.to_uppercase() == "GOTO" && then_tokens.len() > 1 => {
                    // IF condition THEN GOTO line_number
                    if let Token::Number(n) = then_tokens[1] {
                        let line_number = n as u16;
                        return Err(BasicError::GotoJump(line_number));
                    } else {
                        return Err(BasicError::Syntax);
                    }
                }
                _ => {
                    // IF condition THEN statement - execute the statement directly
                    return self.execute_statement(then_tokens, mem, evaluator);
                }
            }
        } else {
            // If condition is false, do nothing and continue
            Ok(true)
        }
    }

    /// Execute FOR statement
    fn execute_for(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        // FOR syntax: FOR variable = start TO end [STEP step]
        if tokens.len() < 5 {
            return Err(BasicError::Syntax);
        }

        // Get variable name (should be Identifier)
        let variable_name = match &tokens[0] {
            Token::Identifier(name) => name.clone(),
            _ => return Err(BasicError::Syntax),
        };

        // Next should be Equal
        if !matches!(tokens[1], Token::Equal) {
            return Err(BasicError::Syntax);
        }

        // Find the TO position - the tokens before TO are the start expression
        let mut to_pos = 2;
        while to_pos < tokens.len() && !matches!(tokens[to_pos], Token::Identifier(ref s) if s.to_uppercase() == "TO") {
            to_pos += 1;
        }

        if to_pos >= tokens.len() {
            return Err(BasicError::Syntax);
        }

        // Evaluate start value
        let start_value = evaluator.evaluate(&tokens[2..to_pos], mem)?;

        // Find the STEP position (if exists) - tokens after TO up to STEP are the end expression
        let mut step_pos = to_pos + 1;
        let mut end_pos = tokens.len();
        while step_pos < tokens.len() {
            if matches!(tokens[step_pos], Token::Identifier(ref s) if s.to_uppercase() == "STEP") {
                end_pos = step_pos;
                break;
            }
            step_pos += 1;
        }

        // Evaluate end value
        let end_value = evaluator.evaluate(&tokens[to_pos + 1..end_pos], mem)?;

        // Evaluate step value (default to 1 if no STEP)
        let step_value = if end_pos < tokens.len() && matches!(tokens[end_pos], Token::Identifier(ref s) if s.to_uppercase() == "STEP") {
            if end_pos + 1 >= tokens.len() {
                return Err(BasicError::Syntax);
            }
            evaluator.evaluate(&tokens[end_pos + 1..], mem)?
        } else {
            Value::Integer(1)
        };

        // Set the variable to the start value
        mem.set_variable(variable_name.clone(), start_value.clone())?;

        // Create FOR loop context
        let current_line = mem.current_line();

        // Find the actual next line in the program execution order
        let next_line = {
            let execution_order = mem.get_execution_order();
            if let Some(current_idx) = execution_order.iter().position(|&line| line == current_line) {
                if current_idx + 1 < execution_order.len() {
                    execution_order[current_idx + 1]
                } else {
                    // This is the last line in the program, use current_line + 1 as fallback
                    current_line + 1
                }
            } else {
                // Fallback to current_line + 1 if not found in execution order
                current_line + 1
            }
        };

        let for_loop = ForLoop::new(
            variable_name,
            start_value,
            end_value,
            step_value,
            current_line,
            next_line,
        );

        // Push onto for stack
        mem.push_for_loop(for_loop);

        Ok(true)
    }

    /// Execute NEXT statement
    fn execute_next(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        // NEXT syntax: NEXT [variable]
        let variable_name = if tokens.is_empty() {
            // No variable specified, use the current FOR loop variable
            match mem.current_for_loop() {
                Some(for_loop) => for_loop.variable_name.clone(),
                None => return Err(BasicError::NextWithoutFor),
            }
        } else {
            // Variable specified
            match &tokens[0] {
                Token::Identifier(name) => name.clone(),
                _ => return Err(BasicError::Syntax),
            }
        };

        // Check if we have a matching FOR loop on the stack
        let for_loop = match mem.current_for_loop() {
            Some(fl) => {
                if fl.variable_name.to_uppercase() != variable_name.to_uppercase() {
                    return Err(BasicError::NextWithoutFor);
                }
                fl.clone()
            }
            None => return Err(BasicError::NextWithoutFor),
        };

        // Get current value of the loop variable
        let current_value = mem.get_variable(&for_loop.variable_name)?;

        // Increment the loop variable by the step value
        // Create a simple expression: variable + step
        let step_tokens = vec![
            Token::Identifier(for_loop.variable_name.clone()),
            Token::Plus,
            match &for_loop.step_value {
                Value::Integer(n) => Token::Number(*n as f64),
                Value::Float(f) => Token::Number(*f),
                _ => return Err(BasicError::TypeMismatch),
            },
        ];

        let new_value = evaluator.evaluate(&step_tokens, mem)?;
        mem.set_variable(for_loop.variable_name.clone(), new_value.clone())?;

        // Check if the loop should continue
        if for_loop.should_continue(&new_value)? {
            // Continue the loop - jump back to the line after the FOR statement
            let jump_to_line = for_loop.next_line;
            return Err(BasicError::GotoJump(jump_to_line));
        } else {
            // Loop is done, pop it from the stack
            mem.pop_for_loop();
        }

        Ok(true)
    }

    /// Execute LOAD statement: LOAD "filename"
    fn execute_load(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<bool> {
        use std::fs;

        if tokens.len() != 1 {
            return Err(BasicError::Syntax);
        }

        let filename = match &tokens[0] {
            Token::String(s) => s,
            _ => return Err(BasicError::Syntax),
        };

        // Clear current program
        mem.clear();

        // Read and parse the file
        let content = fs::read_to_string(filename)
            .map_err(|_| BasicError::Generic(format!("Cannot open file: {}", filename)))?;

        use crate::lexer::Lexer;
        let mut lexer = Lexer::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("'") {
                continue; // Skip empty lines and comments
            }

            match lexer.tokenize(line) {
                Ok(tokens) => {
                    if let Some(line_num) = extract_line_number(&tokens) {
                        mem.store_line(line_num, tokens)?;
                    } else {
                        println!("Warning: Line without line number ignored: {}", line);
                    }
                }
                Err(e) => {
                    return Err(BasicError::Generic(format!("Error in line '{}': {:?}", line, e)));
                }
            }
        }

        println!("PROGRAM LOADED FROM {}", filename);
        Ok(true)
    }

    /// Execute SAVE statement: SAVE "filename"
    fn execute_save(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<bool> {
        use std::fs;
        use std::io::Write;

        if tokens.len() != 1 {
            return Err(BasicError::Syntax);
        }

        let filename = match &tokens[0] {
            Token::String(s) => s,
            _ => return Err(BasicError::Syntax),
        };

        // Get program lines in order
        let execution_order = mem.get_execution_order();
        if execution_order.is_empty() {
            return Err(BasicError::Generic("No program to save".to_string()));
        }

        // Format and write program
        let mut content = String::new();
        for &line_num in &execution_order {
            if let Some(program_line) = mem.get_line(line_num) {
                // Format line: line number + tokens
                content.push_str(&format!("{} ", line_num));
                for token in program_line.tokens.iter().skip(1) { // Skip LineNumber token
                    content.push_str(&format!("{} ", token));
                }
                content.push('\n');
            }
        }

        fs::write(filename, content)
            .map_err(|_| BasicError::Generic(format!("Cannot write to file: {}", filename)))?;

        println!("PROGRAM SAVED TO {}", filename);
        Ok(true)
    }

    /// Execute DATA statement: DATA value1, value2, value3...
    fn execute_data(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<bool> {
        if tokens.is_empty() {
            return Ok(true); // Empty DATA statement is valid
        }

        let mut values = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            // Parse each value in the DATA statement
            match &tokens[i] {
                Token::Number(n) => {
                    values.push(Value::Float(*n));
                    i += 1;
                }
                Token::String(s) => {
                    values.push(Value::String(s.clone()));
                    i += 1;
                }
                Token::Comma => {
                    i += 1; // Skip comma separators
                }
                _ => {
                    return Err(BasicError::Syntax);
                }
            }
        }

        // Add parsed values to memory
        mem.add_data_values(values);
        Ok(true)
    }

    /// Execute READ statement: READ variable1, variable2, variable3...
    fn execute_read(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        _evaluator: &mut ExpressionEvaluator,
    ) -> BasicResult<bool> {
        if tokens.is_empty() {
            return Err(BasicError::Syntax);
        }

        let mut i = 0;
        while i < tokens.len() {
            // Skip commas
            if matches!(tokens[i], Token::Comma) {
                i += 1;
                continue;
            }

            // Get variable name
            let var_name = match &tokens[i] {
                Token::Identifier(name) => name,
                _ => return Err(BasicError::Syntax),
            };

            // Read next value from DATA
            let data_value = match mem.read_data_value() {
                Ok(value) => value,
                Err(BasicError::OutOfData) => {
                    // OUT OF DATA error - return the error to be handled by main loop
                    return Err(BasicError::OutOfData);
                }
                Err(e) => return Err(e),
            };

            // Check type compatibility and store
            if var_name.ends_with('$') {
                // String variable
                match data_value {
                    Value::String(s) => {
                        mem.set_variable(var_name.clone(), Value::String(s))?;
                    }
                    _ => {
                        // Convert non-string to string
                        mem.set_variable(var_name.clone(), Value::String(data_value.to_string()))?;
                    }
                }
            } else {
                // Numeric variable
                if data_value.is_numeric() {
                    mem.set_variable(var_name.clone(), data_value)?;
                } else {
                    return Err(BasicError::TypeMismatch);
                }
            }

            i += 1;
        }

        Ok(true)
    }

    /// Execute RESTORE statement: RESTORE
    fn execute_restore(&mut self, mem: &mut MemoryManager) -> BasicResult<bool> {
        mem.restore_data();
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;
    use crate::runtime::memory::{MemoryManager, Value};

    #[test]
    fn test_let_statement() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test simple LET statement: LET A = 42
        let tokens = vec![
            Token::Let,
            Token::Identifier("A".to_string()),
            Token::Equal,
            Token::Number(42.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result); // Should continue execution

        // Check if variable was stored correctly
        let value = mem.get_variable("A").unwrap();
        assert_eq!(value, &Value::Float(42.0));
    }

    #[test]
    fn test_implicit_let_statement() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test implicit LET: B = 3.14
        let tokens = vec![
            Token::Identifier("B".to_string()),
            Token::Equal,
            Token::Number(3.14),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result);

        // Check if variable was stored correctly
        let value = mem.get_variable("B").unwrap();
        assert_eq!(value, &Value::Float(3.14));
    }

    #[test]
    fn test_let_with_expression() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test LET with expression: LET C = 2 + 3 * 4
        let tokens = vec![
            Token::Let,
            Token::Identifier("C".to_string()),
            Token::Equal,
            Token::Number(2.0),
            Token::Plus,
            Token::Number(3.0),
            Token::Multiply,
            Token::Number(4.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result);

        // Check if variable was stored correctly (2 + 3 * 4 = 14)
        let value = mem.get_variable("C").unwrap();
        assert_eq!(value, &Value::Float(14.0));
    }

    #[test]
    fn test_string_variable_assignment() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test string variable: LET A$ = "HELLO"
        let tokens = vec![
            Token::Let,
            Token::Identifier("A$".to_string()),
            Token::Equal,
            Token::String("HELLO".to_string()),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result);

        // Check if string variable was stored correctly
        let value = mem.get_variable("A$").unwrap();
        assert_eq!(value, &Value::String("HELLO".to_string()));
    }

    #[test]
    fn test_variable_reassignment() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // First assignment: LET X = 10
        let tokens1 = vec![
            Token::Identifier("X".to_string()),
            Token::Equal,
            Token::Number(10.0),
        ];

        executor.execute_statement(&tokens1, &mut mem, &mut evaluator).unwrap();
        assert_eq!(mem.get_variable("X").unwrap(), &Value::Float(10.0));

        // Second assignment: X = X + 5
        let tokens2 = vec![
            Token::Identifier("X".to_string()),
            Token::Equal,
            Token::Identifier("X".to_string()),
            Token::Plus,
            Token::Number(5.0),
        ];

        executor.execute_statement(&tokens2, &mut mem, &mut evaluator).unwrap();
        assert_eq!(mem.get_variable("X").unwrap(), &Value::Float(15.0));
    }

    #[test]
    fn test_let_syntax_errors() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test missing equals: LET A 42
        let tokens1 = vec![
            Token::Let,
            Token::Identifier("A".to_string()),
            Token::Number(42.0),
        ];

        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err(), BasicError::Syntax);

        // Test missing right side: LET A =
        let tokens2 = vec![
            Token::Let,
            Token::Identifier("A".to_string()),
            Token::Equal,
        ];

        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), BasicError::Syntax);

        // Test invalid variable name: LET = 42
        let tokens3 = vec![
            Token::Let,
            Token::Equal,
            Token::Number(42.0),
        ];

        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), BasicError::Syntax);
    }

    #[test]
    fn test_print_statement() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a variable for testing
        mem.set_variable("A".to_string(), crate::runtime::memory::Value::Float(42.0)).unwrap();

        // Test simple PRINT: PRINT 123
        let tokens1 = vec![Token::Print, Token::Number(123.0)];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_ok());

        // Test PRINT variable: PRINT A
        let tokens2 = vec![Token::Print, Token::Identifier("A".to_string())];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_ok());

        // Test PRINT string: PRINT "HELLO"
        let tokens3 = vec![Token::Print, Token::String("HELLO".to_string())];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_ok());

        // Test PRINT expression: PRINT 2 + 3
        let tokens4 = vec![
            Token::Print,
            Token::Number(2.0),
            Token::Plus,
            Token::Number(3.0),
        ];
        let result4 = executor.execute_statement(&tokens4, &mut mem, &mut evaluator);
        assert!(result4.is_ok());
    }

    #[test]
    fn test_print_with_separators() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test PRINT with semicolon: PRINT "HELLO"; "WORLD"
        let tokens1 = vec![
            Token::Print,
            Token::String("HELLO".to_string()),
            Token::Semicolon,
            Token::String("WORLD".to_string()),
        ];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_ok());

        // Test PRINT with comma: PRINT "HELLO", "WORLD"
        let tokens2 = vec![
            Token::Print,
            Token::String("HELLO".to_string()),
            Token::Comma,
            Token::String("WORLD".to_string()),
        ];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_ok());

        // Test PRINT mixed: PRINT "A"; 1, "B"; 2
        let tokens3 = vec![
            Token::Print,
            Token::String("A".to_string()),
            Token::Semicolon,
            Token::Number(1.0),
            Token::Comma,
            Token::String("B".to_string()),
            Token::Semicolon,
            Token::Number(2.0),
        ];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_ok());
    }

    #[test]
    fn test_print_empty() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test empty PRINT (should just print newline)
        let tokens = vec![Token::Print];
        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator);
        assert!(result.is_ok());
    }

    #[test]
    fn test_print_simple_semicolon() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test the simplest case: PRINT "HELLO"; "WORLD"
        // Note: Need to include the PRINT token at the beginning!
        let tokens = vec![
            Token::Print,  // This was missing!
            Token::String("HELLO".to_string()),
            Token::Semicolon,
            Token::String("WORLD".to_string()),
        ];
        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator);
        assert!(result.is_ok());
    }

    #[test]
    fn test_input_syntax_errors() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test empty INPUT
        let tokens1 = vec![Token::Input];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err(), BasicError::Syntax);

        // Test INPUT with prompt but no separator
        let tokens2 = vec![Token::Input, Token::String("Enter name".to_string())];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), BasicError::Syntax);

        // Test INPUT with prompt but no variable
        let tokens3 = vec![
            Token::Input,
            Token::String("Enter name".to_string()),
            Token::Semicolon,
        ];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), BasicError::Syntax);

        // Test INPUT with invalid variable name (number instead of identifier)
        let tokens4 = vec![
            Token::Input,
            Token::String("Enter value".to_string()),
            Token::Semicolon,
            Token::Number(123.0),
        ];
        let result4 = executor.execute_statement(&tokens4, &mut mem, &mut evaluator);
        assert!(result4.is_err());
        assert_eq!(result4.unwrap_err(), BasicError::Syntax);
    }

    #[test]
    fn test_input_prompt_parsing() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // These tests check that the structure is parsed correctly
        // but won't actually read input since that would require manual interaction

        // Test INPUT with semicolon prompt (should add "?")
        let tokens1 = vec![
            Token::Input,
            Token::String("Enter name".to_string()),
            Token::Semicolon,
            Token::Identifier("NAME$".to_string()),
        ];
        // We can't test the actual input without mocking stdin, but we can test that parsing doesn't fail immediately
        // The actual read_line will fail in test environment, but structure should be correct

        // Test INPUT with comma prompt (no additional "?")
        let tokens2 = vec![
            Token::Input,
            Token::String("Enter name".to_string()),
            Token::Comma,
            Token::Identifier("NAME$".to_string()),
        ];

        // Test INPUT without prompt (default "?")
        let tokens3 = vec![
            Token::Input,
            Token::Identifier("NAME$".to_string()),
        ];

        // Test INPUT with numeric variable
        let tokens4 = vec![
            Token::Input,
            Token::Identifier("AGE".to_string()),
        ];

        // All of these should have proper structure, even if they fail at read_line
        // We can't easily test the input reading without mocking stdin
    }

    #[test]
    fn test_goto_statement() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a program line to jump to
        use crate::runtime::memory::ProgramLine;
        mem.program_lines.insert(100, ProgramLine {
            number: 100,
            tokens: vec![Token::Print, Token::String("Target line".to_string())],
        });

        // Test GOTO to existing line
        let tokens = vec![Token::Goto, Token::Number(100.0)];
        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator);

        // Should return GotoJump error (which is actually control flow)
        match result {
            Err(BasicError::GotoJump(line)) => assert_eq!(line, 100),
            _ => panic!("Expected GotoJump error"),
        }

        // Test GOTO to non-existing line
        let tokens2 = vec![Token::Goto, Token::Number(200.0)];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), BasicError::LineNumberNotFound(200));

        // Test GOTO with invalid syntax (no line number)
        let tokens3 = vec![Token::Goto];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), BasicError::Syntax);

        // Test GOTO with invalid line number (not a number)
        let tokens4 = vec![Token::Goto, Token::String("ABC".to_string())];
        let result4 = executor.execute_statement(&tokens4, &mut mem, &mut evaluator);
        assert!(result4.is_err());
        assert_eq!(result4.unwrap_err(), BasicError::Syntax);
    }

    #[test]
    fn test_gosub_return_statement() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a program line to jump to
        use crate::runtime::memory::ProgramLine;
        mem.program_lines.insert(100, ProgramLine {
            number: 100,
            tokens: vec![Token::Print, Token::String("Subroutine".to_string())],
        });

        // Set current line so we have something to return to
        mem.set_current_line(10);

        // Test GOSUB to existing line
        let tokens1 = vec![Token::Gosub, Token::Number(100.0)];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);

        // Should return GosubJump error (which is actually control flow)
        match result1 {
            Err(BasicError::GosubJump(line)) => assert_eq!(line, 100),
            _ => panic!("Expected GosubJump error"),
        }

        // Check that return address was pushed to stack
        assert_eq!(mem.gosub_stack_size(), 1);
        // Note: We can't directly access the stack element, but we know it was pushed

        // Test RETURN with something on stack
        let tokens2 = vec![Token::Return];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);

        // Should return ReturnJump error (which is actually control flow)
        match result2 {
            Err(BasicError::ReturnJump(line)) => assert_eq!(line, 10),
            _ => panic!("Expected ReturnJump error"),
        }

        // Check that stack is now empty
        assert_eq!(mem.gosub_stack_size(), 0);

        // Test RETURN with empty stack (should error)
        let tokens3 = vec![Token::Return];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), BasicError::ReturnWithoutGosub);
    }

    #[test]
    fn test_control_flow_syntax_errors() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test GOSUB with no arguments
        let tokens1 = vec![Token::Gosub];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err(), BasicError::Syntax);

        // Test GOSUB with too many arguments
        let tokens2 = vec![Token::Gosub, Token::Number(100.0), Token::Number(200.0)];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), BasicError::Syntax);

        // Test GOTO with too many arguments
        let tokens3 = vec![Token::Goto, Token::Number(100.0), Token::Number(200.0)];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), BasicError::Syntax);

        // Test GOTO with non-numeric line number
        let tokens4 = vec![Token::Goto, Token::Identifier("LABEL".to_string())];
        let result4 = executor.execute_statement(&tokens4, &mut mem, &mut evaluator);
        assert!(result4.is_err());
        assert_eq!(result4.unwrap_err(), BasicError::Syntax);
    }

    #[test]
    fn test_for_statement_simple() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set current line for testing
        mem.set_current_line(100);

        // Test simple FOR statement: FOR I = 1 TO 5
        let tokens = vec![
            Token::Identifier("I".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Identifier("TO".to_string()),
            Token::Number(5.0),
        ];

        // This should be parsed as a FOR statement, not as tokens for execute_statement
        // We need to include the For token at the beginning
        let for_tokens = vec![
            Token::For,
            Token::Identifier("I".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Identifier("TO".to_string()),
            Token::Number(5.0),
        ];

        let result = executor.execute_statement(&for_tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result); // Should continue execution

        // Check if variable was initialized correctly
        let value = mem.get_variable("I").unwrap();
        assert_eq!(value, &Value::Float(1.0));

        // Check if FOR loop was pushed onto stack
        assert_eq!(mem.for_stack_size(), 1);
        let for_loop = mem.current_for_loop().unwrap();
        assert_eq!(for_loop.variable_name, "I");
        assert_eq!(for_loop.start_value, Value::Float(1.0));
        assert_eq!(for_loop.end_value, Value::Float(5.0));
        assert_eq!(for_loop.step_value, Value::Integer(1));
        assert_eq!(for_loop.current_line, 100);
    }

    #[test]
    fn test_for_statement_with_step() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        mem.set_current_line(200);

        // Test FOR with STEP: FOR J = 10 TO 1 STEP -2
        let tokens = vec![
            Token::Identifier("J".to_string()),
            Token::Equal,
            Token::Number(10.0),
            Token::Identifier("TO".to_string()),
            Token::Number(1.0),
            Token::Identifier("STEP".to_string()),
            Token::Number(-2.0),
        ];

        let for_tokens = vec![
            Token::For,
            Token::Identifier("J".to_string()),
            Token::Equal,
            Token::Number(10.0),
            Token::Identifier("TO".to_string()),
            Token::Number(1.0),
            Token::Identifier("STEP".to_string()),
            Token::Number(-2.0),
        ];

        let result = executor.execute_statement(&for_tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result);

        // Check variable initialization
        let value = mem.get_variable("J").unwrap();
        assert_eq!(value, &Value::Float(10.0));

        // Check FOR loop parameters
        let for_loop = mem.current_for_loop().unwrap();
        assert_eq!(for_loop.variable_name, "J");
        assert_eq!(for_loop.step_value, Value::Float(-2.0));
    }

    #[test]
    fn test_next_statement_continue() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        mem.set_current_line(100);

        // First set up a FOR loop
        let for_tokens = vec![
            Token::For,
            Token::Identifier("I".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Identifier("TO".to_string()),
            Token::Number(3.0),
        ];

        executor.execute_statement(&for_tokens, &mut mem, &mut evaluator).unwrap();

        // Now execute NEXT: NEXT I
        let next_tokens = vec![
            Token::Next,
            Token::Identifier("I".to_string()),
        ];

        let result = executor.execute_statement(&next_tokens, &mut mem, &mut evaluator);

        // Should return GotoJump to continue the loop
        assert!(result.is_err());
        match result.unwrap_err() {
            BasicError::GotoJump(line) => assert_eq!(line, 101), // Should jump to line after FOR
            _ => panic!("Expected GotoJump"),
        }

        // Variable should be incremented
        let value = mem.get_variable("I").unwrap();
        assert_eq!(value, &Value::Float(2.0));

        // FOR loop should still be on stack
        assert_eq!(mem.for_stack_size(), 1);
    }

    #[test]
    fn test_next_statement_complete() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        mem.set_current_line(100);

        // Set up a FOR loop that will complete
        let for_tokens = vec![
            Token::For,
            Token::Identifier("I".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Identifier("TO".to_string()),
            Token::Number(2.0),
        ];

        executor.execute_statement(&for_tokens, &mut mem, &mut evaluator).unwrap();

        // Increment I to 2 (first NEXT)
        let next_tokens = vec![
            Token::Next,
            Token::Identifier("I".to_string()),
        ];
        let _ = executor.execute_statement(&next_tokens, &mut mem, &mut evaluator);

        // Increment I to 3 (second NEXT) - should complete the loop
        let result = executor.execute_statement(&next_tokens, &mut mem, &mut evaluator);
        assert!(result.is_ok()); // Should not return GotoJump
        assert!(result.unwrap()); // Should continue execution

        // Variable should be 3
        let value = mem.get_variable("I").unwrap();
        assert_eq!(value, &Value::Float(3.0));

        // FOR loop should be popped from stack
        assert_eq!(mem.for_stack_size(), 0);
    }

    #[test]
    fn test_next_without_for() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Try NEXT without a FOR loop
        let tokens = vec![
            Token::Next,
            Token::Identifier("I".to_string()),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator);
        assert!(result.is_err());
        // Should be NextWithoutFor because there's no FOR loop on the stack
        assert!(matches!(result.unwrap_err(), BasicError::NextWithoutFor));
    }

    #[test]
    fn test_for_syntax_errors() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test missing TO
        let tokens1 = vec![
            Token::For,
            Token::Identifier("I".to_string()),
            Token::Equal,
            Token::Number(1.0),
        ];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err(), BasicError::Syntax);

        // Test missing variable
        let tokens2 = vec![
            Token::For,
            Token::Equal,
            Token::Number(1.0),
            Token::Identifier("TO".to_string()),
            Token::Number(5.0),
        ];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), BasicError::Syntax);
    }

    #[test]
    fn test_if_statement_true_condition() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a variable
        mem.set_variable("A".to_string(), Value::Integer(10)).unwrap();

        // Test IF with true condition: IF A > 5 THEN B = 20
        let tokens = vec![
            Token::If,
            Token::Identifier("A".to_string()),
            Token::Greater,
            Token::Number(5.0),
            Token::Identifier("THEN".to_string()),
            Token::Identifier("B".to_string()),
            Token::Equal,
            Token::Number(20.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result); // Should continue execution

        // Check if B was set
        let value = mem.get_variable("B").unwrap();
        assert_eq!(value, &Value::Float(20.0));
    }

    #[test]
    fn test_if_statement_false_condition() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a variable
        mem.set_variable("A".to_string(), Value::Integer(3)).unwrap();

        // Test IF with false condition: IF A > 5 THEN B = 20
        let tokens = vec![
            Token::If,
            Token::Identifier("A".to_string()),
            Token::Greater,
            Token::Number(5.0),
            Token::Identifier("THEN".to_string()),
            Token::Identifier("B".to_string()),
            Token::Equal,
            Token::Number(20.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result); // Should continue execution

        // B should not exist (condition was false)
        assert!(mem.get_variable("B").is_err());
    }

    #[test]
    fn test_if_then_goto() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a variable
        mem.set_variable("X".to_string(), Value::Integer(1)).unwrap();

        // Test IF THEN line_number: IF X = 1 THEN 200
        let tokens = vec![
            Token::If,
            Token::Identifier("X".to_string()),
            Token::Equal,
            Token::Number(1.0),
            Token::Identifier("THEN".to_string()),
            Token::Number(200.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator);
        assert!(result.is_err());
        match result.unwrap_err() {
            BasicError::GotoJump(line) => assert_eq!(line, 200),
            _ => panic!("Expected GotoJump"),
        }
    }

    #[test]
    fn test_if_then_goto_keyword() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up a variable
        mem.set_variable("FLAG".to_string(), Value::Integer(0)).unwrap();

        // Test IF THEN GOTO: IF FLAG = 0 THEN GOTO 100
        let tokens = vec![
            Token::If,
            Token::Identifier("FLAG".to_string()),
            Token::Equal,
            Token::Number(0.0),
            Token::Identifier("THEN".to_string()),
            Token::Identifier("GOTO".to_string()),
            Token::Number(100.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator);
        assert!(result.is_err());
        match result.unwrap_err() {
            BasicError::GotoJump(line) => assert_eq!(line, 100),
            _ => panic!("Expected GotoJump"),
        }
    }

    #[test]
    fn test_if_with_string_condition() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Set up string variables
        mem.set_variable("NAME$".to_string(), Value::String("ALICE".to_string())).unwrap();

        // Test IF with string equality
        let tokens = vec![
            Token::If,
            Token::Identifier("NAME$".to_string()),
            Token::Equal,
            Token::String("ALICE".to_string()),
            Token::Identifier("THEN".to_string()),
            Token::Identifier("FOUND".to_string()),
            Token::Equal,
            Token::Number(1.0),
        ];

        let result = executor.execute_statement(&tokens, &mut mem, &mut evaluator).unwrap();
        assert!(result);

        // Check if FOUND was set
        let value = mem.get_variable("FOUND").unwrap();
        assert_eq!(value, &Value::Float(1.0));
    }

    #[test]
    fn test_if_syntax_errors() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test missing THEN
        let tokens1 = vec![
            Token::If,
            Token::Identifier("A".to_string()),
            Token::Greater,
            Token::Number(5.0),
        ];
        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator);
        assert!(result1.is_err());
        assert_eq!(result1.unwrap_err(), BasicError::Syntax);

        // Test empty THEN
        let tokens2 = vec![
            Token::If,
            Token::Identifier("A".to_string()),
            Token::Greater,
            Token::Number(5.0),
            Token::Identifier("THEN".to_string()),
        ];
        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator);
        assert!(result2.is_err());
        assert_eq!(result2.unwrap_err(), BasicError::Syntax);

        // Test IF with no condition
        let tokens3 = vec![
            Token::If,
            Token::Identifier("THEN".to_string()),
            Token::Identifier("A".to_string()),
            Token::Equal,
            Token::Number(1.0),
        ];
        let result3 = executor.execute_statement(&tokens3, &mut mem, &mut evaluator);
        assert!(result3.is_err());
        assert_eq!(result3.unwrap_err(), BasicError::Syntax);
    }

    #[test]
    fn test_if_zero_as_false() {
        let mut executor = StatementExecutor::new();
        let mut mem = MemoryManager::new();
        let mut evaluator = ExpressionEvaluator::new();

        // Test IF with 0 condition (should be false)
        let tokens1 = vec![
            Token::If,
            Token::Number(0.0),
            Token::Identifier("THEN".to_string()),
            Token::Identifier("RESULT".to_string()),
            Token::Equal,
            Token::Number(1.0),
        ];

        let result1 = executor.execute_statement(&tokens1, &mut mem, &mut evaluator).unwrap();
        assert!(result1);

        // RESULT should not exist (0 is false)
        assert!(mem.get_variable("RESULT").is_err());

        // Test IF with non-zero condition (should be true)
        let tokens2 = vec![
            Token::If,
            Token::Number(42.0),
            Token::Identifier("THEN".to_string()),
            Token::Identifier("RESULT".to_string()),
            Token::Equal,
            Token::Number(1.0),
        ];

        let result2 = executor.execute_statement(&tokens2, &mut mem, &mut evaluator).unwrap();
        assert!(result2);

        // RESULT should exist (42 is true)
        let value = mem.get_variable("RESULT").unwrap();
        assert_eq!(value, &Value::Float(1.0));
    }
}