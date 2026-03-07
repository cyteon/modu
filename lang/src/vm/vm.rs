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
        let mut vm = Self {
            chunks,
            stack: Vec::with_capacity(STACK_MAX),
            frames: Vec::with_capacity(FRAMES_MAX),
            globals: HashMap::new(),
        };

        for func in crate::functions::get_functions() {
            vm.globals.insert(func.name.clone(), Value::InternalFn(func));
        }

        vm
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
                if self.frames.len() == 1 {
                    return Ok(());
                }

                continue;
            }

            let instruction = &self.chunks[frame.chunk_id].instructions[frame.ip];
            frame.ip += 1;

            match instruction {
                Instruction::Push(i) => {
                    let v = self.chunks[frame.chunk_id].constants[*i].clone();
                    self.stack.push(v);
                }

                Instruction::StoreGlobal(name) => {
                    let v = self.stack.pop().unwrap();
                    self.globals.insert(name.clone(), v);
                }

                Instruction::LoadGlobal(name) => {
                    let v = self.globals.get(name).cloned().unwrap_or(Value::Null);
                    self.stack.push(v);
                }

                Instruction::Call(argc) => {
                    let callee = self.stack[self.stack.len() - 1 - argc].clone();

                    match callee {
                        Value::InternalFn(func) => {
                            let args: Vec<Value> = self.stack.drain(self.stack.len() - argc..).collect();
                            self.stack.pop();

                            let result = (func.func)(args);
                        }

                        _ => {
                            dbg!(callee);
                        }
                    }
                }

                _ => todo!(),
            }
        }
    }
}