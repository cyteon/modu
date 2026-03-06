use crate::vm::chunk::Chunk;
use super::scope::{ScopeStack, Variable};

pub struct Compiler {
    pub chunks: Vec<Chunk>,
    scope: ScopeStack,
    current_chunk: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunks: vec![Chunk::new("main")],
            scope: ScopeStack::new(),
            current_chunk: 0,
        }
    }

    pub fn compile_program(&mut self, ast: Vec<crate::ast::SpannedExpr>) -> Result<(), String> {
        Ok(())
    }
}