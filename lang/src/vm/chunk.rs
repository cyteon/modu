use super::value::Value;
use super::instruction::Instruction;

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

    pub fn emit(&mut self, instruction: Instruction) -> usize {
        let index = self.instructions.len();
        self.instructions.push(instruction);
        index
    }

    pub fn patch_jump(&mut self, jump: usize) {
        let target = self.instructions.len();

        match &mut self.instructions[jump] {
            Instruction::Jump(offset)
            | Instruction::JumpIfFalse(offset)
            | Instruction::JumpIfTrue(offset) => {
                *offset = target;
            }

            _ => panic!("can only patch jump instructions"),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        for (i, c) in self.constants.iter().enumerate() {
            if c == &value {
                return i;
            }
        }

        let index = self.constants.len();
        self.constants.push(value);
        index
    }
}