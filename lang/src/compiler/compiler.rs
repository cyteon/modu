use crate::vm::chunk::Chunk;
use crate::vm::instruction::Instruction;
use crate::vm::value::Value;
use crate::ast::{SpannedExpr, Expr};

use super::scope::{ScopeStack, Variable};

pub struct Compiler {
    pub chunks: Vec<Chunk>,
    scope: ScopeStack,
    current_chunk: usize,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunks: vec![Chunk::new("main")],
            scope: ScopeStack::new(),
            current_chunk: 0,
        }
    }

    // chunk shit

    fn emit(&mut self, instruction: Instruction) {
        self.chunks[self.current_chunk].emit(instruction);
    }

    fn add_constant(&mut self, value: Value) -> usize {
        self.chunks[self.current_chunk].add_constant(value)
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
            self.compile_expr(expr)?;
        }

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

            Expr::Null => {
                let index = self.add_constant(Value::Null);
                self.emit(Instruction::Push(index));
            }

            _ => todo!(),
        }

        Ok(())
    }
}