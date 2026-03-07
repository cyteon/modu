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

                Instruction::Neg => {
                    let a = self.stack.pop().unwrap_or(Value::Null);
                    self.stack.push(a.neg()?);
                }

                Instruction::Add => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(a.add(&b)?);
                }

                Instruction::Sub => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(a.sub(&b)?);
                }

                Instruction::Mul => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(a.mul(&b)?);
                }

                Instruction::Div => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(a.div(&b)?);
                }

                Instruction::Mod => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(a.r#mod(&b)?);
                }

                Instruction::Call(argc) => {
                    let callee = self.stack[self.stack.len() - 1 - argc].clone();

                    match callee {
                        Value::InternalFn(func) => {
                            let args: Vec<Value> = self.stack.drain(self.stack.len() - argc..).collect();
                            self.stack.pop();

                            match (func.func)(args) {
                                Ok(result) => self.stack.push(result),
                                Err(e) => return Err(format!("error calling {}(): {}", func.name, e)),
                            }
                            
                        }

                        Value::Function { chunk_id, arity } => {
                            if arity != *argc {
                                return Err(format!("expected {} arguments but got {}", arity, argc));
                            }

                            if self.frames.len() >= FRAMES_MAX {
                                return Err("stack overflow".to_string());
                            }

                            let base = self.stack.len() - argc;

                            let extra_locals = self.chunks[chunk_id].locals_count.saturating_sub(*argc);
                            for _ in 0..extra_locals {
                                self.stack.push(Value::Null);
                            }
                            
                            self.frames.push(CallFrame {
                                chunk_id,
                                ip: 0,
                                base,
                            });
                        }

                        _ => {
                            return Err(format!("{} is not a function", callee.type_name()));
                        }
                    }
                }

                Instruction::Return => {
                    let result = self.stack.pop().unwrap_or(Value::Null);
                    let frame = self.frames.pop().unwrap();

                    if frame.base > 0 {
                        self.stack.truncate(frame.base - 1);
                    }
                    

                    self.stack.push(result);
                }

                Instruction::Pop => {
                    self.stack.pop();
                }

                Instruction::StoreGlobal(name) => {
                    let v = self.stack.pop().unwrap_or(Value::Null);
                    self.globals.insert(name.clone(), v);
                }

                Instruction::StoreLocal(slot) => {
                    let v = self.stack.pop().unwrap_or(Value::Null);
                    self.stack[frame.base + slot] = v;
                }

                Instruction::LoadGlobal(name) => {
                    let v = match self.globals.get(name) {
                        Some(v) => v.clone(),
                        None => return Err(format!("undefined variable '{}'", name)),
                    };

                    self.stack.push(v);
                }

                Instruction::LoadLocal(slot) => {
                    let v = self.stack[frame.base + slot].clone();
                    self.stack.push(v);
                }

                _ => {
                    dbg!(instruction);
                    todo!();
                }
            }
        }
    }
}