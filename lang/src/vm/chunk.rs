use crate::vm::value::Value;
use crate::vm::instruction::Instruction;

pub struct Chunk {
    name: String,
    instructions: Vec<Instruction>,
    constants: Vec<Value>,
    locals_count: usize,
}

impl Chunk {

}