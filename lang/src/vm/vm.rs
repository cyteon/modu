use std::collections::HashMap;

use super::chunk::Chunk;
use super::value::Value;

pub struct VM {
    pub chunks: Vec<Chunk>,
    globals: HashMap<String, Value>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        Ok(())
    }   
}