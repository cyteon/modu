use chumsky::span::SimpleSpan;
use std::collections::HashMap;

use crate::vm::chunk::Chunk;
use crate::vm::instruction::Instruction;
use crate::vm::value::Value;
use crate::ast::{SpannedExpr, Expr, AssignOp};
use super::scope::{ScopeStack, Variable};

pub struct Compiler {
    pub global_consts: Vec<String>,
    pub chunks: Vec<Chunk>,
    pub offset: usize,
    scope: ScopeStack,
    current_chunk: usize,
    break_patches: Vec<Vec<usize>>,
    continue_targets: Vec<usize>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            global_consts: Vec::new(),
            chunks: vec![Chunk::new("main")],
            offset: 0,
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
            | Expr::Import { .. }
        )
    }

    fn block_is_void(expr: &Expr) -> bool {
        match expr {
            Expr::Block(exprs) => exprs.last().map(|e| Self::is_void(&e.node)).unwrap_or(true),
            _ => false,
        }
    }

    // chunk shit

    fn emit(&mut self, instruction: Instruction, span: SimpleSpan) {
        self.chunks[self.current_chunk].emit(instruction, span);
    }

    fn emit_jump(&mut self, instruction: Instruction, span: SimpleSpan) -> usize {
        self.chunks[self.current_chunk].emit(instruction, span)
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.chunks[self.current_chunk].add_constant(value)
    }

    fn patch_jump(&mut self, jump: usize) {
        self.chunks[self.current_chunk].patch_jump(jump);
    }

    // scope shit

    fn store_variable(&mut self, name: &str, span: SimpleSpan) {
        if self.scope.in_function() {
            match self.scope.resolve(name) {
                Variable::Local(index) => {
                    self.emit(Instruction::StoreLocal(index), span);
                }

                Variable::Global(_) => {
                    let slot = self.scope.define_local(name);
                    self.emit(Instruction::StoreLocal(slot), span);
                }
            }
        } else {
            self.emit(Instruction::StoreGlobal(name.to_string()), span);
        }
    }

    // compiling shit

    pub fn compile_program(&mut self, ast: Vec<SpannedExpr>) -> Result<(), String> {
        for expr in ast {
            let span = expr.span;
            self.compile_expr(expr.clone())?;

            if !Self::is_void(&expr.node) {
                self.emit(Instruction::Pop, span);
            }
        }

        self.chunks[self.current_chunk].locals_count = self.scope.max_slot;

        Ok(())
    }

    fn compile_expr(&mut self, expr: SpannedExpr) -> Result<(), String> {
        let span = expr.span;

        match &expr.node {
            Expr::Let { name, value } => {
                self.compile_expr(*value.clone())?;
                self.store_variable(name, span);
            }

            Expr::Const { name, value } => {
                self.compile_expr(*value.clone())?;
                
                if self.scope.in_function() {
                    let slot = self.scope.define_const(name);
                    self.emit(Instruction::StoreLocal(slot), span);
                } else {
                    self.global_consts.push(name.clone());
                    self.emit(Instruction::StoreGlobal(name.to_string()), span);
                }
            }

            Expr::Identifier(name) => {
                match self.scope.resolve(name) {
                    Variable::Local(index) => {
                        self.emit(Instruction::LoadLocal(index), span);
                    }

                    Variable::Global(_) => {
                        self.emit(Instruction::LoadGlobal(name.to_string()), span);
                    }
                }
            }

            Expr::Assign { target, value, operator } => {
                if let Expr::Identifier(name) = &target.node {
                    if self.scope.is_const(name) || self.global_consts.contains(name) {
                        return Err(format!("cannot assign to constant '{}'", name));
                    }
                }

                match &target.node {
                    Expr::Identifier(name) => {
                        let var = self.scope.resolve(name);

                        if let Some(op) = operator {
                            match var {
                                Variable::Local(index) => {
                                    self.emit(Instruction::LoadLocal(index), span);
                                }

                                Variable::Global(_) => {
                                    self.emit(Instruction::LoadGlobal(name.to_string()), span);
                                }
                            }

                            self.compile_expr(*value.clone())?;

                            match op {
                                AssignOp::Add => { self.emit(Instruction::Add, span); }
                                AssignOp::Sub => { self.emit(Instruction::Sub, span); }
                                AssignOp::Mul => { self.emit(Instruction::Mul, span); }
                                AssignOp::Div => { self.emit(Instruction::Div, span); }
                                AssignOp::Mod => { self.emit(Instruction::Mod, span); }
                            }  
                        } else {
                            self.compile_expr(*value.clone())?;
                        }

                        match var {
                            Variable::Local(index) => {
                                self.emit(Instruction::StoreLocal(index), span);
                            }

                            Variable::Global(_) => {
                                self.emit(Instruction::StoreGlobal(name.to_string()), span);
                            }
                        }
                    }

                    Expr::IndexAccess { .. } | Expr::PropertyAccess { .. } => {
                        if let Some(op) = operator {
                            self.compile_expr(*target.clone())?;
                            self.compile_expr(*value.clone())?;

                            match op {
                                AssignOp::Add => { self.emit(Instruction::Add, span); }
                                AssignOp::Sub => { self.emit(Instruction::Sub, span); }
                                AssignOp::Mul => { self.emit(Instruction::Mul, span); }
                                AssignOp::Div => { self.emit(Instruction::Div, span); }
                                AssignOp::Mod => { self.emit(Instruction::Mod, span); }
                            }
                        } else {
                            self.compile_expr(*value.clone())?;
                        }

                        self.compile_assign(target)?;
                    }

                    _ => return Err("invalid assignment target".to_string()),
                }
            }

            Expr::Call { callee, args } => {
                let argc = args.len();

                self.compile_expr(*callee.clone())?;
                for arg in args {
                    self.compile_expr(arg.clone())?;
                }

                if let Expr::PropertyAccess { object, property: _ } = &callee.node {
                    let (target_local, target_global) = match &object.node {
                        Expr::Identifier(name) => {
                            match self.scope.resolve(name) {
                                Variable::Local(index) => (Some(index), None),
                                Variable::Global(_) => (None, Some(name.to_string())),
                            }
                        }

                        _ => (None, None),
                    };

                    self.emit(Instruction::CallMethod { argc, target_local, target_global, }, span);
                } else {
                    self.emit(Instruction::Call(argc), span);
                }
            }

            Expr::PropertyAccess { object, property } => {
                self.compile_expr(*object.clone())?;
                self.emit(Instruction::GetProperty(property.clone()), span);
            }


            Expr::IndexAccess { object, index } => {
                self.compile_expr(*object.clone())?;
                self.compile_expr(*index.clone())?;
                self.emit(Instruction::IndexGet, span);
            }

            Expr::Int(i) => {
                let index = self.add_constant(Value::Int(*i));
                self.emit(Instruction::Push(index), span);
            }

            Expr::Float(f) => {
                let index = self.add_constant(Value::Float(*f));
                self.emit(Instruction::Push(index), span);
            }

            Expr::String(s) => {
                let index = self.add_constant(Value::String(s.clone()));
                self.emit(Instruction::Push(index), span);
            }

            Expr::Bool(b) => {
                let index = self.add_constant(Value::Bool(*b));
                self.emit(Instruction::Push(index), span);
            }

            Expr::Array(elements) => {
                for element in elements {
                    self.compile_expr(element.clone())?;
                }

                self.emit(Instruction::MakeArray(elements.len()), span);
            }

            Expr::Object { properties } => {
                for (key, value) in properties {
                    self.compile_expr(value.clone())?;

                    let key_index = self.add_constant(Value::String(key.clone()));
                    self.emit(Instruction::Push(key_index), span);
                }

                self.emit(Instruction::MakeObject(properties.len()), span);
            }
            
            Expr::Null => {
                self.emit(Instruction::PushNull, span);
            }

            Expr::Neg(v) => {
                self.compile_expr(*v.clone())?;
                self.emit(Instruction::Neg, span);
            }

            Expr::Add(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Add, span);
            }

            Expr::Sub(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Sub, span);
            }

            Expr::Mul(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Mul, span);
            }

            Expr::Div(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Div, span);
            }

            Expr::Mod(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Mod, span);
            }

            Expr::Pow(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Pow, span);
            }

            Expr::BitAnd(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::BitAnd, span);
            }

            Expr::BitOr(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::BitOr, span);
            }

            Expr::BitXor(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::BitXor, span);
            }

            Expr::BitShl(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::BitShl, span);
            }

            Expr::BitShr(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::BitShr, span);
            },

            Expr::BitNot(a) => {
                self.compile_expr(*a.clone())?;
                self.emit(Instruction::BitNot, span);
            }

            Expr::Equal(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Eq, span);
            }

            Expr::NotEqual(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Neq, span);
            }

            Expr::LessThan(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Lt, span);
            }

            Expr::LessThanOrEqual(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Lte, span);
            }

            Expr::GreaterThan(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Gt, span);
            }

            Expr::GreaterThanOrEqual(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::Gte, span);
            }

            Expr::In(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::In, span);
            }

            Expr::NotIn(a, b) => {
                self.compile_expr(*a.clone())?;
                self.compile_expr(*b.clone())?;
                self.emit(Instruction::NotIn, span);
            }

            Expr::And(a, b) => {
                self.compile_expr(*a.clone())?;
                let false_jump = self.emit_jump(Instruction::JumpIfFalse(0), span);
                
                self.compile_expr(*b.clone())?;
                let end_jump = self.emit_jump(Instruction::Jump(0), span);

                self.patch_jump(false_jump);
                let idx = self.add_constant(Value::Bool(false));
                self.emit(Instruction::Push(idx), span);

                self.patch_jump(end_jump);
            }

            Expr::Or(a, b) => {
                self.compile_expr(*a.clone())?;
                let true_jump = self.emit_jump(Instruction::JumpIfFalse(0), span);

                let idx = self.add_constant(Value::Bool(true));
                self.emit(Instruction::Push(idx), span);
                let end_jump = self.emit_jump(Instruction::Jump(0), span);

                self.patch_jump(true_jump);
                self.compile_expr(*b.clone())?;

                self.patch_jump(end_jump);
            }

            Expr::Not(v) => {
                self.compile_expr(*v.clone())?;
                self.emit(Instruction::Not, span);
            }

            Expr::Function { name, args, body } => {
                let local_index = self.chunks.len();
                let chunk_id = local_index + self.offset;
                self.chunks.push(Chunk::new(name));

                let saved_chunk = self.current_chunk;
                self.current_chunk = local_index;

                let saved = self.scope.enter_function();
                for arg in args {
                    self.scope.define_local(arg);
                }

                self.compile_expr(*body.clone())?;
                self.emit(Instruction::Return, span);

                let locals_count = self.scope.exit_function(saved);
                self.chunks[local_index].locals_count = locals_count;

                self.current_chunk = saved_chunk;

                let fn_value = Value::Function { chunk_id, arity: args.len() };
                let index = self.add_constant(fn_value);

                self.emit(Instruction::Push(index), span);
                self.store_variable(name, span);
            },

            Expr::InfiniteLoop { body } => {
                let start = self.chunks[self.current_chunk].instructions.len();
                self.break_patches.push(Vec::new());
                self.continue_targets.push(start);

                self.compile_expr(*body.clone())?;

                if !Self::block_is_void(&body.node) {
                    self.emit(Instruction::Pop, span);
                }

                self.emit(Instruction::Jump(start), span);

                let breaks = self.break_patches.pop().unwrap();
                for b in breaks {
                    self.patch_jump(b);
                }

                self.emit(Instruction::PushNull, span);
            }

            Expr::WhileLoop { condition, body } => {
                let start = self.chunks[self.current_chunk].instructions.len();
                self.break_patches.push(Vec::new());
                self.continue_targets.push(start);

                self.compile_expr(*condition.clone())?;
                let exit = self.emit_jump(Instruction::JumpIfFalse(0), span);

                self.compile_expr(*body.clone())?;

                if !Self::block_is_void(&body.node) {
                    self.emit(Instruction::Pop, span);
                }

                self.emit(Instruction::Jump(start), span);

                let breaks = self.break_patches.pop().unwrap();
                for b in breaks {
                    self.patch_jump(b);
                }

                self.patch_jump(exit);
                self.emit(Instruction::PushNull, span);
            }

            Expr::Range { start, end } => {
                self.compile_expr(*start.clone())?;
                self.compile_expr(*end.clone())?;
                self.emit(Instruction::MakeRange { inclusive: false }, span);
            }

            Expr::InclusiveRange { start, end } => {
                self.compile_expr(*start.clone())?;
                self.compile_expr(*end.clone())?;
                self.emit(Instruction::MakeRange { inclusive: true }, span);
            }

            Expr::ForLoop { iterator_name, iterator_range, body } => {
                self.scope.push_scope();

                self.compile_expr(*iterator_range.clone())?;
                let slot_iter = self.scope.define_local("__iter__");
                self.emit(Instruction::StoreLocal(slot_iter), span);

                let zero = self.add_constant(Value::Int(0));
                self.emit(Instruction::Push(zero), span);
                let slot_index = self.scope.define_local("__index__");
                self.emit(Instruction::StoreLocal(slot_index), span);

                self.emit(Instruction::PushNull, span);
                let var = self.scope.define_local(iterator_name);
                self.emit(Instruction::StoreLocal(var), span);

                // loop header
                let start = self.chunks[self.current_chunk].instructions.len();
                self.break_patches.push(Vec::new());
                self.continue_targets.push(start);

                self.emit(Instruction::IterNext {
                    slot_iter,
                    slot_index,
                    slot_var: var,
                }, span);

                let exit = self.emit_jump(Instruction::JumpIfFalse(0), span);
                
                // body
                self.compile_expr(*body.clone())?;

                if !Self::block_is_void(&body.node) {
                    self.emit(Instruction::Pop, span);
                }

                self.emit(Instruction::Jump(start), span);

                self.patch_jump(exit);
                let breaks = self.break_patches.pop().unwrap();
                for b in breaks {
                    self.patch_jump(b);
                }

                self.scope.pop_scope();
                self.emit(Instruction::PushNull, span);
            }

            Expr::Block(exprs) => {
                self.scope.push_scope();

                let last = exprs.len().saturating_sub(1);
                for (i, expr) in exprs.iter().enumerate() {
                    let expr_span = expr.span;
                    self.compile_expr(expr.clone())?;

                    if i != last && !Self::is_void(&expr.node) {
                        self.emit(Instruction::Pop, expr_span);
                    }
                }

                self.scope.pop_scope();
            }

            Expr::Return(v) => {
                self.compile_expr(*v.clone())?;
                self.emit(Instruction::Return, span);
            }

            Expr::Break => {
                let jump = self.emit_jump(Instruction::Jump(0), span);

                if let Some(breaks) = self.break_patches.last_mut() {
                    breaks.push(jump);
                } else {
                    return Err("break outside of loop".to_string()); // this shouldnt happen because of the validator
                }

                self.emit(Instruction::PushNull, span);
            }

            Expr::Continue => {
                match self.continue_targets.last() {
                    Some(target) => {
                        self.emit_jump(Instruction::Jump(*target), span);
                    }

                    None => return Err("continue outside of loop".to_string()),
                }

                self.emit(Instruction::PushNull, span);
            }

            Expr::If(branches) => {
                let mut end_jumps = Vec::new();

                for (condition, body) in branches {
                    match condition {
                        Some(cond) => {
                            self.compile_expr(cond.clone())?;
                            let skip = self.emit_jump(Instruction::JumpIfFalse(0), span);
                            self.compile_expr(body.clone())?;
                            let end = self.emit_jump(Instruction::Jump(0), span);
                            self.patch_jump(skip);
                            end_jumps.push(end);
                        }

                        None => {
                            self.compile_expr(body.clone())?;
                        }
                    }
                }

                // so if statements always leave smth on the stack
                let has_else = branches.last().map(|(c, _)| c.is_none()).unwrap_or(false);
                if !has_else {
                    self.emit(Instruction::PushNull, span);
                }

                for jump in end_jumps {
                    self.patch_jump(jump);
                }
            }

            Expr::Import { name, alias } => {
                self.emit(Instruction::Import { path: name.clone(), alias: alias.clone() }, span);
            }

            Expr::Class { name, methods } => {
                let mut methods_map = HashMap::new();

                for f in methods {
                    if let Expr::Function { name: method_name, args, body } = &f.node {
                        let chunk_id = self.chunks.len() + self.offset;
                        self.chunks.push(Chunk::new(&format!("{}::{}", name, method_name)));
                        
                        let saved_chunk = self.current_chunk;
                        self.current_chunk = chunk_id;

                        let saved_scope = self.scope.enter_function();
                        self.scope.define_local("self");

                        for arg in args {
                            self.scope.define_local(arg);
                        }

                        self.compile_expr(*body.clone())?;

                        if method_name == "init" {
                            self.emit(Instruction::LoadLocal(0), span);
                        } else {
                            self.emit(Instruction::PushNull, span);
                        }

                        self.emit(Instruction::Return, span);

                        let locals_count = self.scope.exit_function(saved_scope);
                        self.chunks[chunk_id].locals_count = locals_count;
                        self.current_chunk = saved_chunk;

                        methods_map.insert(
                            method_name.clone(), 
                            Value::Function { chunk_id, arity: args.len() }
                        );
                    } else {
                        return Err("class body can only contain functions".to_string());
                    }
                }

                let class_value = Value::Class { name: name.clone(), methods: methods_map };
                let index = self.add_constant(class_value);

                self.emit(Instruction::Push(index), span);
                self.store_variable(name, span);
            }

            Expr::Try { try_block, catch_block, catch_var } => {
                self.scope.push_scope();

                let mut var_slot = 0;
                if let Some(var) = catch_var {
                    var_slot = self.scope.define_local(&var);
                }
                
                let setup = self.emit_jump(Instruction::SetupTry(0), span);

                self.compile_expr(*try_block.clone())?;
                if !Self::block_is_void(&try_block.node) {
                    self.emit(Instruction::Pop, span);
                }

                self.emit(Instruction::EndTry, span);
                let jump_end = self.emit_jump(Instruction::Jump(0), span);

                self.patch_jump(setup);

                if let Some(_) = catch_var {
                    self.emit(Instruction::StoreLocal(var_slot), span);
                }

                self.compile_expr(*catch_block.clone())?;
                if !Self::block_is_void(&catch_block.node) {
                    self.emit(Instruction::Pop, span);
                }

                self.patch_jump(jump_end);
                self.emit(Instruction::PushNull, span);

                self.scope.pop_scope();
            }
        }

        Ok(())
    }

    fn compile_assign(&mut self, target: &SpannedExpr) -> Result<(), String> {
        let span = target.span;

        match &target.node {
            Expr::Identifier(name) => {
                match self.scope.resolve(name) {
                    Variable::Local(slot) => self.emit(Instruction::StoreLocal(slot), span),
                    Variable::Global(_) => self.emit(Instruction::StoreGlobal(name.clone()), span),
                }
            }

            Expr::IndexAccess { object, index } => {
                self.compile_expr(*object.clone())?;
                self.compile_expr(*index.clone())?;
                self.emit(Instruction::Rotate3, span);
                self.emit(Instruction::IndexSet, span);
                self.compile_assign(object)?;
            }
    
            Expr::PropertyAccess { object, property } => {
                self.compile_expr(*object.clone())?;
                self.emit(Instruction::Swap, span);
                self.emit(Instruction::SetProperty(property.clone()), span);
                self.compile_assign(object)?;
            },

            _ => return Err("invalid assignment target".to_string()),
        }

        Ok(())
    }
}