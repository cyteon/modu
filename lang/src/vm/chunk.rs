use crate::vm::value::Value;
use crate::vm::instruction::Instruction;

#[derive(Debug)]
pub struct Chunk {
    pub name: String,
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Value>,
    pub locals_count: usize,
}

impl Chunk {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            instructions: Vec::new(),
            constants: Vec::new(),
            locals_count: 0,
        }
    }
}