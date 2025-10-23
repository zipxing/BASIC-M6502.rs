//! String pool management for efficient string storage and garbage collection
//!
//! This module handles string storage, similar to the original BASIC's
//! string space management and garbage collection.

use std::collections::HashMap;

/// String descriptor (3 bytes in original, expanded for Rust)
#[derive(Clone, Debug, PartialEq)]
pub struct StringDescriptor {
    pub length: usize,      // Length of the string
    pub address: usize,     // Address in string pool
}

/// String pool for managing string memory
#[derive(Clone, Debug)]
pub struct StringPool {
    strings: Vec<String>,           // Actual string data
    descriptors: HashMap<usize, StringDescriptor>, // Descriptors by address
    next_address: usize,            // Next available address
    total_memory: usize,            // Total memory limit
}

impl StringPool {
    /// Create a new string pool
    pub fn new(total_memory: usize) -> Self {
        Self {
            strings: Vec::new(),
            descriptors: HashMap::new(),
            next_address: 0,
            total_memory,
        }
    }

    /// Store a string and return its descriptor
    pub fn store_string(&mut self, string: String) -> BasicResult<StringDescriptor> {
        if self.next_address + string.len() > self.total_memory {
            // Try garbage collection first
            self.garbage_collect()?;
            if self.next_address + string.len() > self.total_memory {
                return Err(BasicError::OutOfMemory);
            }
        }

        let address = self.next_address;
        let length = string.len();

        let descriptor = StringDescriptor { length, address };
        self.descriptors.insert(address, descriptor.clone());
        self.strings.push(string);
        self.next_address += length;

        Ok(descriptor)
    }

    /// Get string by descriptor
    pub fn get_string(&self, descriptor: &StringDescriptor) -> Option<&String> {
        self.strings.get(descriptor.address)
    }

    /// Remove unused strings (garbage collection)
    pub fn garbage_collect(&mut self) -> BasicResult<()> {
        // For now, this is a simplified implementation
        // In a full implementation, we would:
        // 1. Mark all strings that are referenced by variables
        // 2. Compact the string space
        // 3. Update all descriptors

        self.compact_strings();
        Ok(())
    }

    /// Compact the string space to remove gaps
    fn compact_strings(&mut self) {
        let mut new_strings = Vec::new();
        let mut new_descriptors = HashMap::new();
        let mut current_address = 0;

        for string in &self.strings {
            let descriptor = StringDescriptor {
                length: string.len(),
                address: current_address,
            };
            new_descriptors.insert(current_address, descriptor);
            new_strings.push(string.clone());
            current_address += string.len();
        }

        self.strings = new_strings;
        self.descriptors = new_descriptors;
        self.next_address = current_address;
    }

    /// Get current memory usage
    pub fn memory_usage(&self) -> usize {
        self.next_address
    }

    /// Get available memory
    pub fn available_memory(&self) -> usize {
        self.total_memory.saturating_sub(self.next_address)
    }
}

use crate::error::{BasicError, BasicResult};