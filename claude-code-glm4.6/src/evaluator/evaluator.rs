//! Expression evaluator for BASIC
//!
//! This module implements the expression evaluation engine, corresponding to
//! the FRMEVL (Formula Evaluation) routine in the original Microsoft BASIC.
//! It uses the operator precedence parsing method with a stack-based approach.

use crate::error::{BasicError, BasicResult};
use crate::runtime::memory::{MemoryManager, Value};
use crate::lexer::Token;

/// Expression evaluator using operator precedence parsing
pub struct ExpressionEvaluator {
    /// Stack for storing intermediate results during evaluation
    #[allow(dead_code)]
    value_stack: Vec<Value>,
    /// Stack for storing operators and their precedence
    #[allow(dead_code)]
    operator_stack: Vec<OperatorFrame>,
    /// Current position in token stream
    position: usize,
}

/// Frame for storing operator information on the stack
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct OperatorFrame {
    /// The operator token
    operator: Token,
    /// Precedence level of the operator
    precedence: u8,
    /// Left-hand side value (already evaluated)
    left_value: Value,
}

impl ExpressionEvaluator {
    /// Create a new expression evaluator
    pub fn new() -> Self {
        Self {
            value_stack: Vec::new(),
            operator_stack: Vec::new(),
            position: 0,
        }
    }

    /// Evaluate an expression from tokens
    ///
    /// Simple recursive descent parser implementation.
    pub fn evaluate(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        self.position = 0;
        let result = self.parse_logical_or(tokens, mem)?;

        // Check that all tokens were consumed
        if self.position < tokens.len() {
            // Check if the remaining token is an unmatched parenthesis
            if tokens[self.position] == Token::RightParen {
                return Err(BasicError::ExpectedRightParen);
            } else {
                return Err(BasicError::ExpectedExpression);
            }
        }

        Ok(result)
    }

    /// Parse logical OR operations
    fn parse_logical_or(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        let mut left = self.parse_logical_and(tokens, mem)?;

        while self.position < tokens.len() {
            match self.current_token(tokens) {
                Some(Token::Or) => {
                    self.position += 1;
                    let right = self.parse_logical_and(tokens, mem)?;
                    left = self.execute_or(&left, &right)?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse logical AND operations
    fn parse_logical_and(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        let mut left = self.parse_expression(tokens, mem)?;

        while self.position < tokens.len() {
            match self.current_token(tokens) {
                Some(Token::And) => {
                    self.position += 1;
                    let right = self.parse_expression(tokens, mem)?;
                    left = self.execute_and(&left, &right)?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse a full expression (handles comparisons, addition and subtraction)
    fn parse_expression(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        let mut left = self.parse_term(tokens, mem)?;

        while self.position < tokens.len() {
            match self.current_token(tokens) {
                Some(Token::Plus) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_addition(&left, &right)?;
                }
                Some(Token::Minus) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_subtraction(&left, &right)?;
                }
                // Comparison operators (same precedence as addition/subtraction)
                Some(Token::Equal) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_equal(&left, &right)?;
                }
                Some(Token::NotEqual) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_not_equal(&left, &right)?;
                }
                Some(Token::Less) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_less(&left, &right)?;
                }
                Some(Token::LessEqual) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_less_equal(&left, &right)?;
                }
                Some(Token::Greater) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_greater(&left, &right)?;
                }
                Some(Token::GreaterEqual) => {
                    self.position += 1;
                    let right = self.parse_term(tokens, mem)?;
                    left = self.execute_greater_equal(&left, &right)?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse a term (handles multiplication and division)
    fn parse_term(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        let mut left = self.parse_factor(tokens, mem)?;

        while self.position < tokens.len() {
            match self.current_token(tokens) {
                Some(Token::Multiply) => {
                    self.position += 1;
                    let right = self.parse_factor(tokens, mem)?;
                    left = self.execute_multiplication(&left, &right)?;
                }
                Some(Token::Divide) => {
                    self.position += 1;
                    let right = self.parse_factor(tokens, mem)?;
                    left = self.execute_division(&left, &right)?;
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse a factor (handles power, unary operators, and primary values)
    fn parse_factor(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        // Handle power operator (right-associative)
        let mut base = self.parse_unary(tokens, mem)?;

        if self.position < tokens.len() && self.current_token(tokens) == Some(&Token::Power) {
            self.position += 1;
            let exponent = self.parse_factor(tokens, mem)?; // Recursive for right-associativity
            base = self.execute_power(&base, &exponent)?;
        }

        Ok(base)
    }

    /// Parse unary operators and primary values
    fn parse_unary(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        if self.position >= tokens.len() {
            return Err(BasicError::ExpectedExpression);
        }

        match self.current_token(tokens) {
            Some(Token::Minus) => {
                self.position += 1;
                let operand = self.parse_unary(tokens, mem)?;
                Ok(Value::Float(-operand.to_float()?))
            }
            Some(Token::Plus) => {
                self.position += 1;
                self.parse_unary(tokens, mem)
            }
            _ => self.evaluate_primary(tokens, mem),
        }
    }

    /// Evaluate a primary value (number, string, variable, function call, or parenthesized expression)
    ///
    /// This corresponds to the EVAL routine in the original BASIC.
    fn evaluate_primary(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        if self.position >= tokens.len() {
            return Err(BasicError::ExpectedExpression);
        }

        let token = &tokens[self.position];
        let result = match token {
            Token::Number(n) => Ok(Value::Float(*n)),
            Token::String(s) => Ok(Value::String(s.clone())),
            Token::Identifier(name) => {
                // Look up variable value
                mem.get_variable(name).cloned()
            }
            Token::LeftParen => {
                // Handle parenthesized expression
                self.position += 1; // Skip '('

                if self.position >= tokens.len() {
                    return Err(BasicError::ExpectedExpression);
                }

                // Evaluate the expression inside parentheses
                let inner_value = self.evaluate_subexpression(tokens, mem)?;

                Ok(inner_value)
            }

            // Handle functions
            Token::Sgn => self.evaluate_unary_function(tokens, mem, |v| {
                let f = v.to_float()?;
                Ok(Value::Float(if f > 0.0 { 1.0 } else if f < 0.0 { -1.0 } else { 0.0 }))
            }),

            Token::Int => self.evaluate_unary_function(tokens, mem, |v| {
                Ok(Value::Float(v.to_float()?.floor()))
            }),

            Token::Abs => self.evaluate_unary_function(tokens, mem, |v| {
                Ok(Value::Float(v.to_float()?.abs()))
            }),

            Token::Sqr => self.evaluate_unary_function(tokens, mem, |v| {
                let f = v.to_float()?;
                if f < 0.0 {
                    return Err(BasicError::IllegalQuantity);
                }
                Ok(Value::Float(f.sqrt()))
            }),

            Token::Rnd => {
                self.position += 1;
                // Simple random number implementation
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                std::time::SystemTime::now().hash(&mut hasher);
                let random = (hasher.finish() % 1000000) as f64 / 1000000.0;
                Ok(Value::Float(random))
            }

            // String functions
            Token::Left => self.evaluate_binary_function(tokens, mem, |str_val, len_val| {
                let s = match str_val {
                    Value::String(s) => s,
                    _ => return Err(BasicError::TypeMismatch),
                };
                let len = len_val.to_float()? as usize;
                if len > s.len() {
                    Ok(Value::String(s))
                } else {
                    Ok(Value::String(s[..len].to_string()))
                }
            }),

            Token::Right => self.evaluate_binary_function(tokens, mem, |str_val, len_val| {
                let s = match str_val {
                    Value::String(s) => s,
                    _ => return Err(BasicError::TypeMismatch),
                };
                let len = len_val.to_float()? as usize;
                if len > s.len() {
                    Ok(Value::String(s))
                } else {
                    Ok(Value::String(s[s.len() - len..].to_string()))
                }
            }),

            Token::Mid => self.evaluate_ternary_function(tokens, mem, |str_val, start_val, len_val| {
                let s = match str_val {
                    Value::String(s) => s,
                    _ => return Err(BasicError::TypeMismatch),
                };
                let start = start_val.to_float()? as usize;
                let len = len_val.to_float()? as usize;

                if start == 0 || start > s.len() {
                    return Ok(Value::String(String::new()));
                }

                let start_idx = start - 1; // BASIC is 1-indexed
                let end_idx = std::cmp::min(start_idx + len, s.len());

                if start_idx >= s.len() {
                    Ok(Value::String(String::new()))
                } else {
                    Ok(Value::String(s[start_idx..end_idx].to_string()))
                }
            }),

            Token::Str => self.evaluate_unary_function(tokens, mem, |v| {
                let f = v.to_float()?;
                // Convert to string, removing decimal part if it's .0
                if f.fract() == 0.0 {
                    Ok(Value::String(format!("{}", f as i64)))
                } else {
                    Ok(Value::String(format!("{}", f)))
                }
            }),

            Token::Val => self.evaluate_unary_function(tokens, mem, |v| {
                let s = match v {
                    Value::String(s) => s,
                    _ => return Err(BasicError::TypeMismatch),
                };
                s.parse::<f64>()
                    .map(Value::Float)
                    .map_err(|_| BasicError::TypeMismatch)
            }),

            Token::Asc => self.evaluate_unary_function(tokens, mem, |v| {
                let s = match v {
                    Value::String(s) => s,
                    _ => return Err(BasicError::TypeMismatch),
                };
                if s.is_empty() {
                    return Err(BasicError::IllegalQuantity);
                }
                Ok(Value::Float(s.chars().next().unwrap() as u8 as f64))
            }),

            Token::Chr => self.evaluate_unary_function(tokens, mem, |v| {
                let f = v.to_float()?;
                if f < 0.0 || f > 255.0 {
                    return Err(BasicError::IllegalQuantity);
                }
                let ch = f as u8 as char;
                Ok(Value::String(ch.to_string()))
            }),

            // Handle unary minus
            Token::Minus => {
                self.position += 1;
                let operand = self.evaluate_primary(tokens, mem)?;
                match operand {
                    Value::Float(f) => Ok(Value::Float(-f)),
                    Value::Integer(i) => Ok(Value::Integer(-i)),
                    Value::String(_) => Err(BasicError::TypeMismatch),
                }
            }

            _ => Err(BasicError::ExpectedExpression),
        };

        // Advance position if successful
        if result.is_ok() && !matches!(token, Token::LeftParen) {
            self.position += 1;
        }

        result
    }

    /// Evaluate a unary function with one argument
    fn evaluate_unary_function<F>(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        func: F,
    ) -> BasicResult<Value>
    where
        F: FnOnce(Value) -> BasicResult<Value>,
    {
        self.position += 1; // Skip function name

        // Check for opening parenthesis
        if self.position >= tokens.len() || tokens[self.position] != Token::LeftParen {
            return Err(BasicError::ExpectedRightParen);
        }

        self.position += 1; // Skip '('

        // Evaluate the argument as a full expression
        let arg_value = self.parse_logical_or(tokens, mem)?;

        // Check for closing parenthesis
        if self.position >= tokens.len() || tokens[self.position] != Token::RightParen {
            return Err(BasicError::ExpectedRightParen);
        }

        self.position += 1; // Skip ')'

        // Apply the function
        func(arg_value)
    }

    /// Evaluate a binary function with two arguments
    fn evaluate_binary_function<F>(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        func: F,
    ) -> BasicResult<Value>
    where
        F: FnOnce(Value, Value) -> BasicResult<Value>,
    {
        self.position += 1; // Skip function name

        // Check for opening parenthesis
        if self.position >= tokens.len() || tokens[self.position] != Token::LeftParen {
            return Err(BasicError::ExpectedRightParen);
        }

        self.position += 1; // Skip '('

        // Evaluate first argument
        let arg1 = self.parse_logical_or(tokens, mem)?;

        // Check for comma
        if self.position >= tokens.len() || tokens[self.position] != Token::Comma {
            return Err(BasicError::Syntax);
        }

        self.position += 1; // Skip ','

        // Evaluate second argument
        let arg2 = self.parse_logical_or(tokens, mem)?;

        // Check for closing parenthesis
        if self.position >= tokens.len() || tokens[self.position] != Token::RightParen {
            return Err(BasicError::ExpectedRightParen);
        }

        self.position += 1; // Skip ')'

        // Apply the function
        func(arg1, arg2)
    }

    /// Evaluate a ternary function with three arguments
    fn evaluate_ternary_function<F>(
        &mut self,
        tokens: &[Token],
        mem: &mut MemoryManager,
        func: F,
    ) -> BasicResult<Value>
    where
        F: FnOnce(Value, Value, Value) -> BasicResult<Value>,
    {
        self.position += 1; // Skip function name

        // Check for opening parenthesis
        if self.position >= tokens.len() || tokens[self.position] != Token::LeftParen {
            return Err(BasicError::ExpectedRightParen);
        }

        self.position += 1; // Skip '('

        // Evaluate first argument
        let arg1 = self.parse_logical_or(tokens, mem)?;

        // Check for first comma
        if self.position >= tokens.len() || tokens[self.position] != Token::Comma {
            return Err(BasicError::Syntax);
        }

        self.position += 1; // Skip ','

        // Evaluate second argument
        let arg2 = self.parse_logical_or(tokens, mem)?;

        // Check for second comma
        if self.position >= tokens.len() || tokens[self.position] != Token::Comma {
            return Err(BasicError::Syntax);
        }

        self.position += 1; // Skip ','

        // Evaluate third argument
        let arg3 = self.parse_logical_or(tokens, mem)?;

        // Check for closing parenthesis
        if self.position >= tokens.len() || tokens[self.position] != Token::RightParen {
            return Err(BasicError::ExpectedRightParen);
        }

        self.position += 1; // Skip ')'

        // Apply the function
        func(arg1, arg2, arg3)
    }

    /// Execute a pending operation from the operator stack
    #[allow(dead_code)]
    fn execute_pending_operation(&mut self) -> BasicResult<()> {
        if self.operator_stack.is_empty() || self.value_stack.len() < 2 {
            return Err(BasicError::Syntax);
        }

        let frame = self.operator_stack.pop().unwrap();
        let right_value = self.value_stack.pop().unwrap();

        // Execute the operation with the stored left value and current right value
        let result = self.execute_binary_operation(&frame.left_value, &frame.operator, &right_value)?;

        // Push the result back to the value stack
        self.value_stack.push(result);

        Ok(())
    }

    /// Execute a binary operation between two values
    #[allow(dead_code)]
    fn execute_binary_operation(&self, left: &Value, op: &Token, right: &Value) -> BasicResult<Value> {
        match op {
            // Arithmetic operations
            Token::Plus => self.execute_addition(left, right),
            Token::Minus => self.execute_subtraction(left, right),
            Token::Multiply => self.execute_multiplication(left, right),
            Token::Divide => self.execute_division(left, right),
            Token::Power => self.execute_power(left, right),

            // Comparison operations
            Token::Equal => self.execute_equal(left, right),
            Token::NotEqual => self.execute_not_equal(left, right),
            Token::Less => self.execute_less(left, right),
            Token::LessEqual => self.execute_less_equal(left, right),
            Token::Greater => self.execute_greater(left, right),
            Token::GreaterEqual => self.execute_greater_equal(left, right),

            // Logical operations
            Token::And => self.execute_and(left, right),
            Token::Or => self.execute_or(left, right),

            _ => Err(BasicError::Syntax),
        }
    }

    /// Execute addition (+)
    fn execute_addition(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        match (left, right) {
            (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
            _ => {
                let left_f = left.to_float()?;
                let right_f = right.to_float()?;
                Ok(Value::Float(left_f + right_f))
            }
        }
    }

    /// Execute subtraction (-)
    fn execute_subtraction(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let left_f = left.to_float()?;
        let right_f = right.to_float()?;
        Ok(Value::Float(left_f - right_f))
    }

    /// Execute multiplication (*)
    fn execute_multiplication(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let left_f = left.to_float()?;
        let right_f = right.to_float()?;
        Ok(Value::Float(left_f * right_f))
    }

    /// Execute division (/)
    fn execute_division(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let left_f = left.to_float()?;
        let right_f = right.to_float()?;
        if right_f == 0.0 {
            return Err(BasicError::DivisionByZero);
        }
        Ok(Value::Float(left_f / right_f))
    }

    /// Execute power (^)
    fn execute_power(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let left_f = left.to_float()?;
        let right_f = right.to_float()?;
        Ok(Value::Float(left_f.powf(right_f)))
    }

    /// Execute equal comparison (=)
    fn execute_equal(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let result = match (left, right) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            _ => {
                let left_f = left.to_float()?;
                let right_f = right.to_float()?;
                (left_f - right_f).abs() < f64::EPSILON
            }
        };
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Execute not equal comparison (<>)
    fn execute_not_equal(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let equal_result = self.execute_equal(left, right)?;
        if let Value::Float(f) = equal_result {
            Ok(Value::Float(if f == 1.0 { 0.0 } else { 1.0 }))
        } else {
            Err(BasicError::Syntax)
        }
    }

    /// Execute less than comparison (<)
    fn execute_less(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let result = match (left, right) {
            (Value::String(a), Value::String(b)) => a < b,
            _ => {
                let left_f = left.to_float()?;
                let right_f = right.to_float()?;
                left_f < right_f
            }
        };
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Execute less than or equal comparison (<=)
    fn execute_less_equal(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let result = match (left, right) {
            (Value::String(a), Value::String(b)) => a <= b,
            _ => {
                let left_f = left.to_float()?;
                let right_f = right.to_float()?;
                left_f <= right_f
            }
        };
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Execute greater than comparison (>)
    fn execute_greater(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let result = match (left, right) {
            (Value::String(a), Value::String(b)) => a > b,
            _ => {
                let left_f = left.to_float()?;
                let right_f = right.to_float()?;
                left_f > right_f
            }
        };
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Execute greater than or equal comparison (>=)
    fn execute_greater_equal(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let result = match (left, right) {
            (Value::String(a), Value::String(b)) => a >= b,
            _ => {
                let left_f = left.to_float()?;
                let right_f = right.to_float()?;
                left_f >= right_f
            }
        };
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Execute logical AND
    fn execute_and(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let left_f = left.to_float()?;
        let right_f = right.to_float()?;
        let result = (left_f != 0.0) && (right_f != 0.0);
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Execute logical OR
    fn execute_or(&self, left: &Value, right: &Value) -> BasicResult<Value> {
        let left_f = left.to_float()?;
        let right_f = right.to_float()?;
        let result = (left_f != 0.0) || (right_f != 0.0);
        Ok(Value::Float(if result { 1.0 } else { 0.0 }))
    }

    /// Get the current token
    fn current_token<'a>(&self, tokens: &'a [Token]) -> Option<&'a Token> {
        tokens.get(self.position)
    }

    /// Evaluate a subexpression (used for parentheses)
    /// Creates a temporary evaluator to avoid interfering with current state
    fn evaluate_subexpression(&mut self, tokens: &[Token], mem: &mut MemoryManager) -> BasicResult<Value> {
        let mut sub_evaluator = ExpressionEvaluator::new();
        let start_pos = self.position;

        // Find the matching closing parenthesis
        let mut paren_depth = 1;
        let mut end_pos = start_pos;

        while end_pos < tokens.len() && paren_depth > 0 {
            match &tokens[end_pos] {
                Token::LeftParen => paren_depth += 1,
                Token::RightParen => paren_depth -= 1,
                _ => {}
            }
            end_pos += 1;
        }

        if paren_depth != 0 {
            return Err(BasicError::ExpectedRightParen);
        }

        // Evaluate the subexpression
        let sub_tokens = &tokens[start_pos..end_pos - 1]; // Exclude closing paren
        let result = sub_evaluator.evaluate(sub_tokens, mem)?;

        // Update position past the closing parenthesis
        self.position = end_pos;

        Ok(result)
    }

    /// Peek at the next token without advancing
    #[allow(dead_code)]
    fn peek_token<'a>(&self, tokens: &'a [Token]) -> Option<&'a Token> {
        tokens.get(self.position + 1)
    }
}

impl Default for ExpressionEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn create_test_evaluator() -> (ExpressionEvaluator, MemoryManager, Lexer) {
        (ExpressionEvaluator::new(), MemoryManager::new(), Lexer::new())
    }

    fn evaluate_expression(expr: &str) -> BasicResult<Value> {
        let (mut evaluator, mut mem, mut lexer) = create_test_evaluator();
        // Use parentheses to prevent line number parsing
        let tokens = lexer.tokenize(&format!("({})", expr))?;
        // Remove the outer parentheses for evaluation
        if tokens.len() >= 2 && tokens[0] == Token::LeftParen && tokens[tokens.len()-1] == Token::RightParen {
            let inner_tokens = &tokens[1..tokens.len()-1];
            evaluator.evaluate(inner_tokens, &mut mem)
        } else {
            evaluator.evaluate(&tokens, &mut mem)
        }
    }

    #[test]
    fn test_simple_arithmetic() {
        assert_eq!(evaluate_expression("2 + 3").unwrap(), Value::Float(5.0));
        assert_eq!(evaluate_expression("10 - 4").unwrap(), Value::Float(6.0));
        assert_eq!(evaluate_expression("3 * 4").unwrap(), Value::Float(12.0));
        assert_eq!(evaluate_expression("15 / 3").unwrap(), Value::Float(5.0));
    }

    #[test]
    fn test_operator_precedence() {
        assert_eq!(evaluate_expression("2 + 3 * 4").unwrap(), Value::Float(14.0));
        assert_eq!(evaluate_expression("(2 + 3) * 4").unwrap(), Value::Float(20.0));
        assert_eq!(evaluate_expression("2 * 3 + 4").unwrap(), Value::Float(10.0));
    }

    #[test]
    fn test_power_operator() {
        assert_eq!(evaluate_expression("2 ^ 3").unwrap(), Value::Float(8.0));
        assert_eq!(evaluate_expression("2 ^ 3 ^ 2").unwrap(), Value::Float(512.0)); // Right associative
    }

    #[test]
    fn test_comparisons() {
        assert_eq!(evaluate_expression("5 = 5").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("5 <> 3").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("5 < 3").unwrap(), Value::Float(0.0));
        assert_eq!(evaluate_expression("5 <= 5").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("5 > 3").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("5 >= 5").unwrap(), Value::Float(1.0));
    }

    #[test]
    fn test_logical_operations() {
        assert_eq!(evaluate_expression("1 AND 1").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("1 AND 0").unwrap(), Value::Float(0.0));
        assert_eq!(evaluate_expression("1 OR 0").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("0 OR 0").unwrap(), Value::Float(0.0));
    }

    #[test]
    fn test_unary_functions() {
        assert_eq!(evaluate_expression("SGN(5)").unwrap(), Value::Float(1.0));
        assert_eq!(evaluate_expression("SGN(0)").unwrap(), Value::Float(0.0));
        assert_eq!(evaluate_expression("SGN(-5)").unwrap(), Value::Float(-1.0));

        assert_eq!(evaluate_expression("INT(3.7)").unwrap(), Value::Float(3.0));
        assert_eq!(evaluate_expression("ABS(-5)").unwrap(), Value::Float(5.0));
        assert_eq!(evaluate_expression("SQR(9)").unwrap(), Value::Float(3.0));
    }

    #[test]
    fn test_string_concatenation() {
        assert_eq!(evaluate_expression("\"HELLO\" + \" \" + \"WORLD\"").unwrap(),
                   Value::String("HELLO WORLD".to_string()));
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(evaluate_expression("-5").unwrap(), Value::Float(-5.0));
        assert_eq!(evaluate_expression("-(3 + 2)").unwrap(), Value::Float(-5.0));
    }

    #[test]
    fn test_variables() {
        let (mut evaluator, mut mem, mut lexer) = create_test_evaluator();

        // Set up a variable
        mem.set_variable("X".to_string(), Value::Float(42.0)).unwrap();

        let tokens = lexer.tokenize("X * 2").unwrap();
        let result = evaluator.evaluate(&tokens, &mut mem).unwrap();
        assert_eq!(result, Value::Float(84.0));
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(evaluate_expression("(2 + 3) * 4 - 1").unwrap(), Value::Float(19.0));
        assert_eq!(evaluate_expression("2 + 3 * (4 - 1)").unwrap(), Value::Float(11.0));
    }

    #[test]
    fn test_division_by_zero() {
        assert!(matches!(evaluate_expression("5 / 0"), Err(BasicError::DivisionByZero)));
    }

    #[test]
    fn test_sqrt_negative() {
        assert!(matches!(evaluate_expression("SQR(-4)"), Err(BasicError::IllegalQuantity)));
    }

    #[test]
    fn test_mismatched_parentheses() {
        assert!(matches!(evaluate_expression("(2 + 3"), Err(BasicError::ExpectedRightParen)));
        assert!(matches!(evaluate_expression("2 + 3)"), Err(BasicError::ExpectedRightParen)));
    }

    #[test]
    fn test_random_function() {
        // Just test that RND returns a valid float
        let result = evaluate_expression("RND").unwrap();
        match result {
            Value::Float(f) => assert!(f >= 0.0 && f < 1.0),
            _ => panic!("RND should return a float"),
        }
    }

    #[test]
    fn test_string_functions() {
        // Test LEFT$
        assert_eq!(evaluate_expression("LEFT$(\"HELLO\", 3)").unwrap(),
                   Value::String("HEL".to_string()));
        assert_eq!(evaluate_expression("LEFT$(\"HI\", 10)").unwrap(),
                   Value::String("HI".to_string()));

        // Test RIGHT$
        assert_eq!(evaluate_expression("RIGHT$(\"HELLO\", 3)").unwrap(),
                   Value::String("LLO".to_string()));
        assert_eq!(evaluate_expression("RIGHT$(\"HI\", 10)").unwrap(),
                   Value::String("HI".to_string()));

        // Test MID$
        assert_eq!(evaluate_expression("MID$(\"HELLO\", 2, 3)").unwrap(),
                   Value::String("ELL".to_string()));
        assert_eq!(evaluate_expression("MID$(\"HELLO\", 1, 2)").unwrap(),
                   Value::String("HE".to_string()));
        assert_eq!(evaluate_expression("MID$(\"HELLO\", 10, 3)").unwrap(),
                   Value::String("".to_string()));

        // Test STR$
        assert_eq!(evaluate_expression("STR$(123)").unwrap(),
                   Value::String("123".to_string()));
        assert_eq!(evaluate_expression("STR$(123.5)").unwrap(),
                   Value::String("123.5".to_string()));

        // Test VAL
        assert_eq!(evaluate_expression("VAL(\"456\")").unwrap(),
                   Value::Float(456.0));
        assert_eq!(evaluate_expression("VAL(\"456.5\")").unwrap(),
                   Value::Float(456.5));

        // Test ASC
        assert_eq!(evaluate_expression("ASC(\"A\")").unwrap(),
                   Value::Float(65.0));

        // Test CHR$
        assert_eq!(evaluate_expression("CHR$(65)").unwrap(),
                   Value::String("A".to_string()));
    }
}