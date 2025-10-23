//! Microsoft BASIC 6502 - Rust Implementation
//!
//! A modern Rust implementation of the classic Microsoft BASIC interpreter
//! originally written for the 6502 microprocessor.
//!
//! # Features
//!
//! - Complete BASIC language support
//! - Expression evaluation with operator precedence
//! - Variable and array management
//! - String handling with garbage collection
//! - Built-in mathematical and string functions
//! - Interactive REPL interface
//!
//! # Quick Start
//!
//! ```rust
//! use basic_m6502_rust::{MemoryManager, ExpressionEvaluator, StatementExecutor};
//!
//! let mut mem = MemoryManager::new();
//! let mut evaluator = ExpressionEvaluator::new();
//! let mut executor = StatementExecutor::new();
//! // Now you can evaluate BASIC expressions or execute statements
//! ```

pub mod lexer;
pub mod parser;
pub mod evaluator;
pub mod runtime;
pub mod statements;
pub mod functions;
pub mod error;
pub mod utils;

// Re-export commonly used types
pub use error::BasicError;
pub use lexer::Token;
pub use runtime::memory::{MemoryManager, Value, Variable, Array};
pub use evaluator::ExpressionEvaluator;
pub use statements::StatementExecutor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_creation() {
        let mem = MemoryManager::new();
        assert!(mem.variables().is_empty());
    }
}
