use std::collections::HashMap;

use super::chunk::Chunk;
use super::value::Value;
use super::instruction::Instruction;

struct CallFrame {
    chunk_id: usize,
    ip: usize,
    base: usize, // so we can do like base + 0 etc for local vars
}

pub struct VM {
    pub chunks: Vec<Chunk>,
    pub stack: Vec<Value>,
    pub frames: Vec<CallFrame>,
    pub globals: HashMap<String, Value>,
}

const STACK_MAX: usize = 2048;
const FRAMES_MAX: usize = 256;

impl VM {
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self {
            chunks,
            stack: Vec::with_capacity(STACK_MAX),
            frames: Vec::with_capacity(FRAMES_MAX),
            globals: HashMap::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let locals_count = self.chunks[0].locals_count;

        self.frames.push(CallFrame {
            chunk_id: 0,
            ip: 0,
            base: 0,
        });

        for _ in 0..locals_count {
            self.stack.push(Value::Null);
        }

        self.execute()
    }
    
    fn execute(&mut self) -> Result<(), String> {
        loop {
            let frame = self.frames.last_mut().unwrap();

            if frame.ip >= self.chunks[frame.chunk_id].instructions.len() {
                continue;
            }

            let instruction = &self.chunks[frame.chunk_id].instructions[frame.ip];
            frame.ip += 1;

            match instruction {
                Instruction::Push(i) => {
                    let v = self.chunks[frame.chunk_id].constants[*i].clone();
                    self.stack.push(v);
                }

                _ => todo!(),
            }
        }
    }
}