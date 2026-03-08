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
            vm.globals.insert(func.name.clone(), Value::BuiltinFn(func));
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

                Instruction::PushNull => {
                    self.stack.push(Value::Null);
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

                Instruction::Eq => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(a == b));
                }

                Instruction::Neq => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(a != b));
                }

                Instruction::Lt => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(a < b));
                }

                Instruction::Lte => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(a <= b));
                }

                Instruction::Gt => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(a > b));
                }

                Instruction::Gte => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(a >= b));
                }

                Instruction::In => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(b.contains(&a)?));
                }

                Instruction::NotIn => {
                    let b = self.stack.pop().unwrap_or(Value::Null);
                    let a = self.stack.pop().unwrap_or(Value::Null);

                    self.stack.push(Value::Bool(!b.contains(&a)?));
                }

                Instruction::Call(argc) => {
                    let callee = self.stack[self.stack.len() - 1 - argc].clone();

                    match callee {
                        Value::BuiltinFn(func) => {
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

                Instruction::CallMethod { argc, target_local, target_global } => {
                    let callee = self.stack[self.stack.len() - 1 - argc].clone();

                    match callee {
                        Value::NativeFn(func) => {
                            let args = self.stack.drain(self.stack.len() - argc..).collect();
                            self.stack.pop();
                            let obj = self.stack.pop().unwrap_or(Value::Null);

                            match (func.func)(obj, args) {
                                Ok(result) => {
                                    if let Some(replace_self) = result.1 {
                                        if let Some(target) = target_local {
                                            let frame = self.frames.last_mut().unwrap();
                                            self.stack[frame.base + target] = replace_self;
                                        } else if let Some(target) = target_global {
                                            self.globals.insert(target.clone(), replace_self);
                                        }
                                    }

                                    self.stack.push(result.0);
                                }
                                Err(e) => return Err(format!("error calling {}(): {}", func.name, e)),
                            }
                        }

                        // if you import a package, then like uuid.v7()
                        Value::BuiltinFn(func) => {
                            let args = self.stack.drain(self.stack.len() - argc..).collect();
                            self.stack.pop();

                            match (func.func)(args) {
                                Ok(result) => self.stack.push(result),
                                Err(e) => return Err(format!("error calling {}(): {}", func.name, e)),
                            }
                        }

                        _ => return Err(format!("{} is not a method", callee.type_name())),
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

                Instruction::MakeArray(len) => {
                    let mut elements = Vec::with_capacity(*len);

                    for _ in 0..*len {
                        let element = self.stack.pop().unwrap_or(Value::Null);
                        elements.push(element);
                    }

                    elements.reverse();
                    self.stack.push(Value::Array(elements));
                }

                Instruction::MakeObject(len) => {
                    let mut properties = HashMap::with_capacity(*len);

                    for _ in 0..*len {
                        let key = self.stack.pop().unwrap_or(Value::Null);
                        let value = self.stack.pop().unwrap_or(Value::Null);

                        let key = match key {
                            Value::String(s) => s,
                            _ => return Err(format!("object property keys must be strings, got {}", key.type_name())),
                        };

                        properties.insert(key, value);
                    }

                    self.stack.push(Value::Object(properties));
                }

                Instruction::IndexGet => {
                    let index = self.stack.pop().unwrap_or(Value::Null);
                    let target = self.stack.pop().unwrap_or(Value::Null);

                    match target {
                        Value::Array(elements) => {
                            let index = match index {
                                Value::Int(i) => i,
                                _ => return Err(format!("expected int index for array but got {}", index.type_name())),
                            };

                            if index < 0 || (index as usize) >= elements.len() {
                                self.stack.push(Value::Null);
                            } else {
                                self.stack.push(elements[index as usize].clone());
                            }
                        }

                        Value::Object(properties) => {
                            let key = match index {
                                Value::String(s) => s,
                                _ => return Err(format!("object property keys must be strings, got {}", index.type_name())),
                            };

                            let value = properties.get(&key).cloned().unwrap_or(Value::Null);
                            self.stack.push(value);
                        }

                        Value::String(s) => todo!(),

                        _ => return Err(format!("{} is not indexable", target.type_name())),
                    }
                }

                Instruction::IndexSet => {
                    let op = self.stack.pop().unwrap_or(Value::Null);
                    let value = self.stack.pop().unwrap_or(Value::Null);
                    let index = self.stack.pop().unwrap_or(Value::Null);
                    let target = self.stack.pop().unwrap_or(Value::Null);

                    let result = match op {
                        Value::Null => {
                            match (target, index) {
                                (Value::Array(mut elements), Value::Int(i)) => {
                                    if i < 0 || (i as usize) >= elements.len() {
                                        return Err(format!("index out of bounds: {}", i));
                                    }

                                    elements[i as usize] = value;
                                    Value::Array(elements)
                                }

                                (t, i) => return Err(format!("cannot index {} with {}", t.type_name(), i.type_name())),
                            }
                        }

                        _ => {
                            dbg!(op);
                            todo!()
                        }
                    };

                    self.stack.push(result);
                }

                Instruction::GetProperty(name) => {
                    let target = self.stack.last().unwrap_or(&Value::Null).clone();

                    match target {
                        Value::Object(properties) => {
                            let value = match properties.get(name) {
                                Some(v) => {
                                    self.stack.pop();
                                    self.stack.push(v.clone());
                                }
                                None => {
                                    let method = match crate::natives::object::get_fn(name.to_string()) {
                                        Some(m) => m,
                                        None => return Err(format!("undefined property '{}' on object", name)),
                                    };

                                    self.stack.push(Value::NativeFn(method));
                                }
                            };
                        }

                        Value::String(_) => {
                            let method = match crate::natives::string::get_fn(name.to_string()) {
                                Some(m) => m,
                                None => return Err(format!("undefined property '{}' on string", name)),
                            };

                            self.stack.push(Value::NativeFn(method));
                        }

                        Value::Int(_) => {
                            let method = match crate::natives::int::get_fn(name.to_string()) {
                                Some(m) => m,
                                None => return Err(format!("undefined property '{}' on {}", name, target.type_name())),
                            };

                            self.stack.push(Value::NativeFn(method));
                        }

                        Value::Float(_) => {
                            let method = match crate::natives::float::get_fn(name.to_string()) {
                                Some(m) => m,
                                None => return Err(format!("undefined property '{}' on float", name)),
                            };

                            self.stack.push(Value::NativeFn(method));
                        }

                        Value::Array(_) => {
                            let method = match crate::natives::array::get_fn(name.to_string()) {
                                Some(m) => m,
                                None => return Err(format!("undefined property '{}' on array", name)),
                            };

                            self.stack.push(Value::NativeFn(method));
                        }

                        _ => return Err(format!("cannot get property '{}' on {}", name, target.type_name())),
                    }
                }

                Instruction::MakeRange { inclusive } => {
                    let end = self.stack.pop().unwrap_or(Value::Null);
                    let start = self.stack.pop().unwrap_or(Value::Null);

                    match (start, end) {
                        (Value::Int(a), Value::Int(b)) => {
                            if *inclusive {
                                self.stack.push(Value::Range { start: a, end: b, inclusive: true });
                            } else {
                                self.stack.push(Value::Range { start: a, end: b, inclusive: false });
                            }
                        }

                        _ => return Err("range bounds must be ints".to_string()),
                    }
                }

                Instruction::IterNext { slot_iter, slot_index, slot_var } => {
                    let iter = self.stack[frame.base + slot_iter].clone();
                    let index = match self.stack[frame.base + slot_index].clone() {
                        Value::Null => Value::Int(0),
                        v => v,
                    };

                    match iter {
                        Value::Range { start, end, inclusive } => {
                            let index = match index {
                                Value::Int(i) => i,
                                _ => return Err(format!("expected int index for range but got {}", index.type_name())),
                            };

                            let next = start + index;

                            if (inclusive && next > end) || (!inclusive && next >= end) {
                                self.stack.push(Value::Bool(false));
                            } else {
                                let frame = self.frames.last_mut().unwrap();
                                self.stack[frame.base + slot_index] = Value::Int(index + 1);
                                self.stack[frame.base + slot_var] = Value::Int(next);
                                self.stack.push(Value::Bool(true));
                            }
                        }

                        Value::Array(elements) => {
                            let index = match index {
                                Value::Int(i) => i,
                                _ => return Err(format!("expected int index for array but got {}", index.type_name())),
                            };

                            if index < 0 || (index as usize) >= elements.len() {
                                self.stack.push(Value::Bool(false));
                            } else {
                                let frame = self.frames.last_mut().unwrap();
                                self.stack[frame.base + slot_index] = Value::Int(index + 1);
                                self.stack[frame.base + slot_var] = elements[index as usize].clone();
                                self.stack.push(Value::Bool(true));
                            }
                        }

                        _ => return Err(format!("{} is not iterable", iter.type_name())),
                    }
                }

                Instruction::Import { path, alias } => {
                    if path.starts_with("std/") {
                        let path = path.strip_prefix("std/").unwrap().to_string();

                        if let Some(module) = crate::stdlib::get(&path) {
                            if let Some(alias) = alias {
                                if alias == "*" {
                                    if let Value::Object(properties) = module {
                                        for (key, value) in properties {
                                            self.globals.insert(key, value);
                                        }
                                    } else {
                                        unreachable!();
                                    }
                                } else {
                                    self.globals.insert(alias.clone(), module);
                                }
                            } else {
                                self.globals.insert(path.clone(), module);
                            }
                        } else {
                            return Err(format!("unknown stdlib module '{}'", path));
                        }
                    } else {
                        todo!();
                    }
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

                Instruction::JumpIfFalse(offset) => {
                    let condition = self.stack.pop().unwrap_or(Value::Null);

                    if !condition.truthy() {
                        frame.ip = *offset;
                    }
                }

                Instruction::Jump(offset) => {
                    frame.ip = *offset;
                }

                _ => {
                    dbg!(instruction);
                    todo!();
                }
            }
        }
    }
}