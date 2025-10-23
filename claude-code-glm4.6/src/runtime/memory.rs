//! Core memory management structures
//!
//! This module defines the main data structures for managing
//! BASIC program state, corresponding to the original memory layout.

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use crate::lexer::Token;
use crate::error::{BasicError, BasicResult};

/// Value types supported in BASIC
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Integer(i16),           // Integer value
    Float(f64),             // Floating point value (using IEEE-754 instead of custom format)
    String(String),         // String value
}

impl Value {
    /// Get the type of this value
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Integer(_) => ValueType::Integer,
            Value::Float(_) => ValueType::Float,
            Value::String(_) => ValueType::String,
        }
    }

    /// Check if this is a numeric value
    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Float(_))
    }

    /// Check if this is a string value
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Convert to float (for numeric operations)
    pub fn to_float(&self) -> BasicResult<f64> {
        match self {
            Value::Integer(n) => Ok(*n as f64),
            Value::Float(f) => Ok(*f),
            Value::String(_) => Err(BasicError::TypeMismatch),
        }
    }

    /// Convert to integer (truncating if necessary)
    pub fn to_integer(&self) -> BasicResult<i16> {
        match self {
            Value::Integer(n) => Ok(*n),
            Value::Float(f) => Ok(*f as i16),
            Value::String(_) => Err(BasicError::TypeMismatch),
        }
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        match self {
            Value::Integer(n) => n.to_string(),
            Value::Float(f) => {
                // Format to match original BASIC output
                if f.fract() == 0.0 {
                    format!("{}.0", f)
                } else {
                    format!("{}", f)
                }
            }
            Value::String(s) => s.clone(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(flt) => write!(f, "{}", flt),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

/// Value type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Integer,
    Float,
    String,
}

/// Variable storage structure
#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,        // Variable name (1-2 chars + optional $ or %)
    pub value: Value,
}

impl Variable {
    /// Create a new variable
    pub fn new(name: String, value: Value) -> Self {
        Self { name, value }
    }

    /// Check if this is a string variable (ends with $)
    pub fn is_string_variable(&self) -> bool {
        self.name.ends_with('$')
    }

    /// Check if this is an integer variable (ends with %)
    pub fn is_integer_variable(&self) -> bool {
        self.name.ends_with('%')
    }

    /// Get the base name without type suffix
    pub fn base_name(&self) -> &str {
        if self.is_string_variable() {
            &self.name[..self.name.len()-1]
        } else if self.is_integer_variable() {
            &self.name[..self.name.len()-1]
        } else {
            &self.name
        }
    }
}

/// Array storage structure
#[derive(Clone, Debug)]
pub struct Array {
    pub name: String,
    pub dimensions: Vec<usize>,  // Size of each dimension
    pub data: Vec<Value>,        // Data stored in row-major order
}

impl Array {
    /// Create a new array with given dimensions
    pub fn new(name: String, dimensions: Vec<usize>) -> Self {
        let total_elements = dimensions.iter().product();
        let data = vec![Value::Float(0.0); total_elements]; // Default to 0.0

        Self {
            name,
            dimensions,
            data,
        }
    }

    /// Get the total number of elements in this array
    pub fn total_elements(&self) -> usize {
        self.data.len()
    }

    /// Convert multi-dimensional indices to linear index (row-major order)
    pub fn indices_to_linear(&self, indices: &[usize]) -> BasicResult<usize> {
        if indices.len() != self.dimensions.len() {
            return Err(BasicError::BadSubscript);
        }

        let mut linear = 0;
        let mut multiplier = 1;

        // Calculate row-major order: offset = i1*D2*D3*... + i2*D3*... + ... + in
        for (i, &dim_size) in indices.iter().rev().enumerate() {
            if dim_size >= self.dimensions[self.dimensions.len() - 1 - i] {
                return Err(BasicError::BadSubscript);
            }

            if i > 0 {
                multiplier *= self.dimensions[self.dimensions.len() - i];
            }
            linear += dim_size * multiplier;
        }

        Ok(linear)
    }

    /// Get value at given indices
    pub fn get(&self, indices: &[usize]) -> BasicResult<&Value> {
        let linear = self.indices_to_linear(indices)?;
        Ok(&self.data[linear])
    }

    /// Set value at given indices
    pub fn set(&mut self, indices: &[usize], value: Value) -> BasicResult<()> {
        let linear = self.indices_to_linear(indices)?;
        self.data[linear] = value;
        Ok(())
    }
}

/// Program line structure
#[derive(Clone, Debug)]
pub struct ProgramLine {
    pub number: u16,
    pub tokens: Vec<Token>,
}

impl ProgramLine {
    /// Create a new program line
    pub fn new(number: u16, tokens: Vec<Token>) -> Self {
        Self { number, tokens }
    }
}

/// FOR loop context for stack management
#[derive(Clone, Debug)]
pub struct ForLoop {
    pub variable_name: String,
    pub start_value: Value,
    pub end_value: Value,
    pub step_value: Value,
    pub current_line: u16,
    pub next_line: u16,  // Line number after the FOR statement
}

impl ForLoop {
    /// Create a new FOR loop context
    pub fn new(variable_name: String, start: Value, end: Value, step: Value, current_line: u16, next_line: u16) -> Self {
        Self {
            variable_name,
            start_value: start,
            end_value: end,
            step_value: step,
            current_line,
            next_line,
        }
    }

    /// Check if the loop should continue
    pub fn should_continue(&self, current_value: &Value) -> BasicResult<bool> {
        let step_float = self.step_value.to_float()?;
        let current_float = current_value.to_float()?;
        let end_float = self.end_value.to_float()?;

        Ok(if step_float > 0.0 {
            current_float <= end_float
        } else if step_float < 0.0 {
            current_float >= end_float
        } else {
            false // Step of 0 creates infinite loop
        })
    }
}

/// GOSUB return stack entry
#[derive(Clone, Debug)]
pub struct GosubReturn {
    pub line_number: u16,
    pub statement_index: usize,
}

/// Main memory manager for BASIC interpreter
#[derive(Clone, Debug)]
pub struct MemoryManager {
    // Program storage (corresponds to TXTTAB in original)
    pub program_lines: BTreeMap<u16, ProgramLine>,

    // Variables (corresponds to VARTAB)
    pub variables: HashMap<String, Variable>,

    // Arrays (corresponds to ARYTAB)
    pub arrays: HashMap<String, Array>,

    // Execution state
    pub current_line: Option<u16>,       // Current executing line
    pub data_pointer: usize,             // Pointer to current DATA element
    pub data_values: Vec<Value>,         // DATA statement values
    pub for_stack: Vec<ForLoop>,         // FOR loop stack
    pub gosub_stack: Vec<GosubReturn>,   // GOSUB return stack

    // String management (simplified for now)
    string_memory: Vec<String>,
}

impl MemoryManager {
    /// Create a new memory manager
    pub fn new() -> Self {
        Self {
            program_lines: BTreeMap::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            current_line: None,
            data_pointer: 0,
            data_values: Vec::new(),
            for_stack: Vec::new(),
            gosub_stack: Vec::new(),
            string_memory: Vec::new(),
        }
    }

    /// Clear all memory (NEW command)
    pub fn clear(&mut self) {
        self.program_lines.clear();
        self.variables.clear();
        self.arrays.clear();
        self.current_line = None;
        self.data_pointer = 0;
        self.data_values.clear();
        self.for_stack.clear();
        self.gosub_stack.clear();
    }

    /// Store a program line
    pub fn store_line(&mut self, line_number: u16, tokens: Vec<Token>) -> BasicResult<()> {
        // Remove existing line with same number if it exists
        if self.program_lines.contains_key(&line_number) {
            self.program_lines.remove(&line_number);
        }

        // If this is an empty line (just line number), delete it
        if tokens.len() == 1 {
            return Ok(());
        }

        // Store the new line
        let line = ProgramLine::new(line_number, tokens);
        self.program_lines.insert(line_number, line);
        Ok(())
    }

    /// Get a program line
    pub fn get_line(&self, line_number: u16) -> Option<&ProgramLine> {
        self.program_lines.get(&line_number)
    }

    /// List all program lines
    pub fn list_program(&self) {
        for (line_num, line) in &self.program_lines {
            print!("{} ", line_num);
            // Skip the first token (LineNumber) and print the rest
            for token in line.tokens.iter().skip(1) {
                print!("{} ", token);
            }
            println!();
        }
    }

    /// Get variable value
    pub fn get_variable(&self, name: &str) -> BasicResult<&Value> {
        self.variables
            .get(name)
            .map(|v| &v.value)
            .ok_or_else(|| BasicError::VariableNotFound(name.to_string()))
    }

    /// Set variable value
    pub fn set_variable(&mut self, name: String, value: Value) -> BasicResult<()> {
        let variable = Variable::new(name.clone(), value);
        self.variables.insert(name, variable);
        Ok(())
    }

    /// Get array
    pub fn get_array(&self, name: &str) -> BasicResult<&Array> {
        self.arrays
            .get(name)
            .ok_or_else(|| BasicError::ArrayNotFound(name.to_string()))
    }

    /// Create or get array
    pub fn get_or_create_array(&mut self, name: String, dimensions: Vec<usize>) -> BasicResult<&mut Array> {
        if !self.arrays.contains_key(&name) {
            let array = Array::new(name.clone(), dimensions);
            self.arrays.insert(name.clone(), array);
        }
        self.arrays.get_mut(&name)
            .ok_or_else(|| BasicError::ArrayNotFound(name))
    }

    /// Get all variables (for testing and debugging)
    pub fn variables(&self) -> &HashMap<String, Variable> {
        &self.variables
    }

    /// Get all arrays
    pub fn arrays(&self) -> &HashMap<String, Array> {
        &self.arrays
    }

    /// Find the next line number after the given one
    pub fn find_next_line(&self, current_line: u16) -> Option<u16> {
        self.program_lines
            .range(current_line + 1..)
            .next()
            .map(|(&num, _)| num)
    }

    /// Get program execution order
    pub fn get_execution_order(&self) -> Vec<u16> {
        self.program_lines.keys().copied().collect()
    }

    /// Get current line number
    pub fn current_line(&self) -> u16 {
        self.current_line.unwrap_or(0)
    }

    /// Set current line number
    pub fn set_current_line(&mut self, line: u16) {
        self.current_line = Some(line);
    }

    /// Push return address onto GOSUB stack
    pub fn push_gosub_return(&mut self, line: u16) {
        self.gosub_stack.push(GosubReturn {
            line_number: line,
            statement_index: 0, // Simplified - always return to start of line
        });
    }

    /// Pop return address from GOSUB stack
    pub fn pop_gosub_return(&mut self) -> Option<u16> {
        self.gosub_stack.pop().map(|ret| ret.line_number)
    }

    /// Get program lines reference
    pub fn program_lines(&self) -> &BTreeMap<u16, ProgramLine> {
        &self.program_lines
    }

    /// Get GOSUB stack size (for testing)
    pub fn gosub_stack_size(&self) -> usize {
        self.gosub_stack.len()
    }

    /// Push a FOR loop context onto the for_stack
    pub fn push_for_loop(&mut self, for_loop: ForLoop) {
        self.for_stack.push(for_loop);
    }

    /// Pop a FOR loop context from the for_stack
    pub fn pop_for_loop(&mut self) -> Option<ForLoop> {
        self.for_stack.pop()
    }

    /// Get the current FOR loop context (top of stack)
    pub fn current_for_loop(&self) -> Option<&ForLoop> {
        self.for_stack.last()
    }

    /// Get the current FOR loop context as mutable
    pub fn current_for_loop_mut(&mut self) -> Option<&mut ForLoop> {
        self.for_stack.last_mut()
    }

    /// Get the size of the for_stack
    pub fn for_stack_size(&self) -> usize {
        self.for_stack.len()
    }

    /// Clear the for_stack
    pub fn clear_for_stack(&mut self) {
        self.for_stack.clear();
    }

    /// Add values from DATA statement to data storage
    pub fn add_data_values(&mut self, values: Vec<Value>) {
        self.data_values.extend(values);
    }

    /// Read next value from DATA storage
    pub fn read_data_value(&mut self) -> BasicResult<Value> {
        if self.data_pointer >= self.data_values.len() {
            return Err(BasicError::OutOfData);
        }
        let value = self.data_values[self.data_pointer].clone();
        self.data_pointer += 1;
        Ok(value)
    }

    /// Reset data pointer to beginning (RESTORE statement)
    pub fn restore_data(&mut self) {
        self.data_pointer = 0;
    }

    /// Get all data values (for debugging)
    pub fn get_data_values(&self) -> &Vec<Value> {
        &self.data_values
    }

    /// Get current data pointer position
    pub fn get_data_pointer(&self) -> usize {
        self.data_pointer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_value_conversions() {
        let int_val = Value::Integer(42);
        let float_val = Value::Float(3.14);
        let string_val = Value::String("hello".to_string());

        assert_eq!(int_val.to_float().unwrap(), 42.0);
        assert_eq!(float_val.to_integer().unwrap(), 3);
        assert!(string_val.to_float().is_err());
        assert_eq!(int_val.to_string(), "42");
        assert_eq!(float_val.to_string(), "3.14");
    }

    #[test]
    fn test_array_indices() {
        let mut array = Array::new("TEST".to_string(), vec![3, 4]); // 3x4 array

        // Test valid indices
        assert!(array.indices_to_linear(&[0, 0]).is_ok());
        assert!(array.indices_to_linear(&[2, 3]).is_ok());

        // Test invalid indices
        assert!(array.indices_to_linear(&[3, 0]).is_err()); // First index out of bounds
        assert!(array.indices_to_linear(&[0, 4]).is_err()); // Second index out of bounds
        assert!(array.indices_to_linear(&[0, 0, 0]).is_err()); // Too many indices

        // Test linear to multidimensional mapping
        let linear = array.indices_to_linear(&[1, 2]).unwrap();
        assert_eq!(array.get(&[1, 2]).unwrap(), &Value::Float(0.0));

        array.set(&[1, 2], Value::Integer(99)).unwrap();
        assert_eq!(array.get(&[1, 2]).unwrap(), &Value::Integer(99));
    }

    #[test]
    fn test_memory_manager() {
        let mut mem = MemoryManager::new();

        // Test variable storage
        mem.set_variable("A$".to_string(), Value::String("test".to_string())).unwrap();
        mem.set_variable("B".to_string(), Value::Integer(42)).unwrap();

        assert_eq!(mem.get_variable("A$").unwrap(), &Value::String("test".to_string()));
        assert_eq!(mem.get_variable("B").unwrap(), &Value::Integer(42));

        // Test program line storage
        let tokens = vec![Token::Print, Token::String("Hello".to_string())];
        mem.store_line(10, tokens).unwrap();

        assert!(mem.get_line(10).is_some());
        assert!(mem.get_line(20).is_none());
    }

    #[test]
    fn test_for_loop() {
        let for_loop = ForLoop::new(
            "I".to_string(),
            Value::Integer(1),
            Value::Integer(10),
            Value::Integer(1),
            100,
            101, // next line after the FOR statement
        );

        assert!(for_loop.should_continue(&Value::Integer(1)).unwrap());
        assert!(for_loop.should_continue(&Value::Integer(10)).unwrap());
        assert!(!for_loop.should_continue(&Value::Integer(11)).unwrap());
    }
}