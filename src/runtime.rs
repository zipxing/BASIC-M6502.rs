use std::collections::HashMap;
use crate::program::Program;
use crate::value::Value;

/// Virtual machine state: holds program and variables.
#[derive(Default)]
pub struct Vm {
    pub program: Program,
    pub vars: HashMap<String, Value>,
}

impl Vm {
    pub fn new() -> Self { Self { program: Program::default(), vars: HashMap::new() } }
}

