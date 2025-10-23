//! Memory management for the BASIC interpreter
//!
//! This module contains the core data structures for managing
//! program memory, variables, arrays, and strings, corresponding
//! to the memory layout of the original Microsoft BASIC.

pub mod memory;
pub mod string_pool;

pub use memory::{MemoryManager, Value, Variable, Array, ProgramLine, ForLoop};
pub use string_pool::{StringPool, StringDescriptor};