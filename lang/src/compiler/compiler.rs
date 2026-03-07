use crate::vm::chunk::Chunk;
use crate::vm::instruction::Instruction;
use crate::vm::value::Value;
use crate::ast::{SpannedExpr, Expr, AssignOp};

use super::scope::{ScopeStack, Variable};

pub struct Compiler {
    pub chunks: Vec<Chunk>,
    scope: ScopeStack,
    current_chunk: usize,
    break_patches: Vec<Vec<usize>>,
    continue_targets: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunks: vec![Chunk::new("main")],
            scope: ScopeStack::new(),
            current_chunk: 0,
            break_patches: Vec::new(),
            continue_targets: Vec::new(),
        }
    }

    fn is_void(expr: &Expr) -> bool {
        matches!(
            expr,
            Expr::Let { .. }
            | Expr::Assign { .. }
            | Expr::Function { .. }
        )
    }

    // chunk shit

    fn emit(&mut self, instruction: Instruction) {
        self.chunks[self.current_chunk].emit(instruction);
    }

    fn emit_jump(&mut self, instruction: Instruction) -> usize {
        self.chunks[self.current_chunk].emit(instruction)
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.chunks[self.current_chunk].add_constant(value)
    }

    fn patch_jump(&mut self, jump: usize) {
        self.chunks[self.current_chunk].patch_jump(jump);
    }

    // scope shit

    fn store_variable(&mut self, name: &str) {
        if self.scope.in_function() {
            match self.scope.resolve(name) {
                Variable::Local(index) => {
                    self.emit(Instruction::StoreLocal(index));
                }

                Variable::Global(_) => {
                    let slot = self.scope.define_local(name);
                    self.emit(Instruction::StoreLocal(slot));
                }
            }
        } else {
            self.emit(Instruction::StoreGlobal(name.to_string()));
        }
    }

    // compiling shit

    pub fn compile_program(&mut self, ast: Vec<SpannedExpr>) -> Result<(), String> {
        for expr in ast {
            self.compile_expr(expr.clone())?;

            if !Self::is_void(&expr.node) {
                self.emit(Instruction::Pop);
            }
        }

        self.chunks[self.current_chunk].locals_count = self.scope.max_slot;

        Ok(())
    }

    fn compile_expr(&mut self, expr: SpannedExpr) -> Result<(), String> {
        match &expr.node {
            Expr::Let { name, value } => {
                self.compile_expr(*value.clone())?;
                self.store_variable(name);
            }

            Expr::Identifier(name) => {
                match self.scope.resolve(name) {
                    Variable::Local(index) => {
                        self.emit(Instruction::LoadLocal(index));
                    }

                    Variable::Global(_) => {
                        self.emit(Instruction::LoadGlobal(name.to_string()));
                    }
                }
            }

            Expr::Assign { target, value, operator } => {
                match &target.node {
                    Expr::Identifier(name) => {
                        let var = self.scope.resolve(name);

                        if let Some(op) = operator {
                            match var {
                                Variable::Local(index) => {
                                    self.emit(Instruction::LoadLocal(index));
                                }

                                Variable::Global(_) => {
                                    self.emit(Instruction::LoadGlobal(name.to_string()));
                                }
                            }

                            self.compile_expr(*value.clone())?;

                            match op {
                                AssignOp::Add => { self.emit(Instruction::Add); }
                                AssignOp::Sub => { self.emit(Instruction::Sub); }
                                AssignOp::Mul => { self.emit(Instruction::Mul); }
                                AssignOp::Div => { self.emit(Instruction::Div); }
                                AssignOp::Mod => { self.emit(Instruction::Mod); }
                            }  
                        } else {
                            self.compile_expr(*value.clone())?;
                        }

                        match var {
                            Variable::Local(index) => {
                                self.emit(Instruction::StoreLocal(index));
                            }

                            Variable::Global(_) => {
                                self.emit(Instruction::StoreGlobal(name.to_string()));
                            }
                        }
                    }

                    Expr::IndexAccess { object, index } => todo!(),

                    Expr::PropertyAccess { object, property } => todo!(),

                    _ => return Err("invalid assignment target".to_string()),
                }
            }

            Expr::Call { callee, args } => {
                self.compile_expr(*callee.clone())?;

                let argc = args.len();
                for arg in args {
                    self.compile_expr(arg.clone())?;
                }

                self.emit(Instruction::Call(argc));
            }

            Expr::Int(i) => {
                let index = self.add_constant(Value::Int(*i));
                self.emit(Instruction::Push(index));
            }

            Expr::Float(f) => {
                let index = self.add_constant(Value::Float(*f));
                self.emit(Instruction::Push(index));
            }

            Expr::String(s) => {
                let index = self.add_constant(Value::String(s.clone()));
                self.emit(Instruction::Push(index));
            }

            Expr::Bool(b) => {
                let index = self.add_constant(Value::Bool(*b));
                self.emit(Instruction::Push(index));
            }

            Expr::Array(elements) => {
                for element in elements {
                    self.compile_expr(element.clone())?;
                }

                self.emit(Instruction::MakeArray(elements.len()));
            }
            
            Expr::Null => {
                self.emit(Instruction::PushNull);
            }

            Expr::Neg(v) => {
                self.compile_expr(*v.clone())?;
                self.emit(Instruction::Neg);
            }

            Expr::Add(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Add);
            }

            Expr::Sub(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Sub);
            }

            Expr::Mul(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Mul);
            }

            Expr::Div(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Div);
            }

            Expr::Mod(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Mod);
            }

            Expr::Equal(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Eq);
            }

            Expr::NotEqual(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Neq);
            }

            Expr::LessThan(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Lt);
            }

            Expr::LessThanOrEqual(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Lte);
            }

            Expr::GreaterThan(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Gt);
            }

            Expr::GreaterThanOrEqual(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Gte);
            }

            Expr::Function { name, args, body } => {
                let chunk_id = self.chunks.len();
                self.chunks.push(Chunk::new(name));

                let saved_chunk = self.current_chunk;
                self.current_chunk = chunk_id;

                let saved = self.scope.enter_function();
                for arg in args {
                    self.scope.define_local(arg);
                }

                self.compile_expr(*body.clone())?;
                self.emit(Instruction::Return);

                let locals_count = self.scope.exit_function(saved);
                self.chunks[chunk_id].locals_count = locals_count;

                self.current_chunk = saved_chunk;

                let fn_value = Value::Function { chunk_id, arity: args.len() };
                let index = self.add_constant(fn_value);

                self.emit(Instruction::Push(index));
                self.store_variable(name);
            },

            Expr::InfiniteLoop { body } => {
                let start = self.chunks[self.current_chunk].instructions.len();
                self.break_patches.push(Vec::new());
                self.continue_targets.push(start);

                self.compile_expr(*body.clone())?;
                self.emit(Instruction::Jump(start));

                let breaks = self.break_patches.pop().unwrap();
                for b in breaks {
                    self.patch_jump(b);
                }

                self.emit(Instruction::PushNull);
            }

            Expr::WhileLoop { condition, body } => {
                let start = self.chunks[self.current_chunk].instructions.len();
                self.break_patches.push(Vec::new());
                self.continue_targets.push(start);

                self.compile_expr(*condition.clone())?;
                let exit = self.emit_jump(Instruction::JumpIfFalse(0));

                self.compile_expr(*body.clone())?;
                self.emit(Instruction::Jump(start));

                let breaks = self.break_patches.pop().unwrap();
                for b in breaks {
                    self.patch_jump(b);
                }

                self.patch_jump(exit);
                self.emit(Instruction::PushNull);
            }

            Expr::Range { start, end } => {
                self.compile_expr(*start.clone())?;
                self.compile_expr(*end.clone())?;
;
                self.emit(Instruction::MakeRange { inclusive: false });
            }

            Expr::InclusiveRange { start, end } => {
                self.compile_expr(*start.clone())?;
                self.compile_expr(*end.clone())?;
                self.emit(Instruction::MakeRange { inclusive: true });
            }

            Expr::ForLoop { iterator_name, iterator_range, body } => {
                self.scope.push_scope();

                self.compile_expr(*iterator_range.clone())?;
                let slot_iter = self.scope.define_local("__iter__");
                self.emit(Instruction::StoreLocal(slot_iter));

                let zero = self.add_constant(Value::Int(0));
                self.emit(Instruction::Push(zero));
                let slot_index = self.scope.define_local("__index__");
                self.emit(Instruction::StoreLocal(slot_index));

                self.emit(Instruction::PushNull);
                let var = self.scope.define_local(iterator_name);
                self.emit(Instruction::StoreLocal(var));

                // loop header
                let start = self.chunks[self.current_chunk].instructions.len();
                self.break_patches.push(Vec::new());
                self.continue_targets.push(start);

                self.emit(Instruction::IterNext {
                    slot_iter,
                    slot_index,
                    slot_var: var,
                });

                let exit = self.emit_jump(Instruction::JumpIfFalse(0));
                
                // body
                self.compile_expr(*body.clone())?;
                self.emit(Instruction::Jump(start));

                self.patch_jump(exit);
                let breaks = self.break_patches.pop().unwrap();
                for b in breaks {
                    self.patch_jump(b);
                }

                self.scope.pop_scope();
                self.emit(Instruction::PushNull);
            }

            Expr::Block(exprs) => {
                self.scope.push_scope();

                let last = exprs.len().saturating_sub(1);
                for (i, expr) in exprs.iter().enumerate() {
                    self.compile_expr(expr.clone())?;

                    if i != last && !Self::is_void(&expr.node) {
                        self.emit(Instruction::Pop);
                    }
                }

                self.scope.pop_scope();
            }

            Expr::Return(v) => {
                self.compile_expr(*v.clone())?;
                self.emit(Instruction::Return);
            }

            Expr::Break => {
                let jump = self.emit_jump(Instruction::Jump(0));

                if let Some(breaks) = self.break_patches.last_mut() {
                    breaks.push(jump);
                } else {
                    return Err("break outside of loop".to_string()); // this shouldnt happen because of the validator
                }

                self.emit(Instruction::PushNull);
            }

            Expr::Continue => {
                match self.continue_targets.last() {
                    Some(target) => {
                        let jump = self.emit_jump(Instruction::Jump(*target));
                    }

                    None => return Err("continue outside of loop".to_string()),
                }

                self.emit(Instruction::PushNull);
            }

            Expr::If(branches) => {
                let mut end_jumps = Vec::new();
                let mut has_else = false;

                for (condition, body) in branches {
                    match condition {
                        Some(cond) => {
                            self.compile_expr(cond.clone())?;
                            let skip = self.emit_jump(Instruction::JumpIfFalse(0));
                            self.compile_expr(body.clone())?;
                            let end = self.emit_jump(Instruction::Jump(0));
                            self.patch_jump(skip);
                            end_jumps.push(end);
                        }

                        None => {
                            has_else = true;
                            self.compile_expr(body.clone())?;
                        }
                    }
                }

                for jump in end_jumps {
                    self.patch_jump(jump);
                }
            }

            v => {
                dbg!(v);
                todo!();
            }
        }

        Ok(())
    }
}