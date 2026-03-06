use crate::vm::value::Value;
use crate::vm::instruction::Instruction;

pub struct Chunk {
    name: String,
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    locals_count: usize,
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