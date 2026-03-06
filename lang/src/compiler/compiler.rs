use crate::vm::chunk::Chunk;
use crate::compiler::scope::ScopeStack;

pub struct Compiler {
    chunks: Vec<Chunk>,
    scope_stack: Vec<ScopeStack>,
    current_chunk: usize,
}


impl Compiler {
    
}