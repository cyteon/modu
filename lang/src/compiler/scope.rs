use std::collections::HashMap;

pub struct Scope {
    locals: HashMap<String, usize>,
    depth: usize,
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
    next_slot: usize,
    max_slot: usize,
    function_depth: usize,
}

pub enum Variable {
    Local(usize),
    Global(String),
}

impl ScopeStack {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            next_slot: 0,
            max_slot: 0,
            function_depth: 0,
        }
    }

    pub fn in_function(&self) -> bool {
        self.function_depth > 0
    }

    pub fn resolve(&self, name: &str) -> Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(&slot) = scope.locals.get(name) {
                return Variable::Local(slot);
            }
        }

        Variable::Global(name.to_string())
    }

    pub fn define_local(&mut self, name: &str) -> usize {
        let slot = self.next_slot;
        self.next_slot += 1;

        if self.next_slot > self.max_slot {
            self.max_slot = self.next_slot;
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.locals.insert(name.to_string(), slot);
        }

        slot
    }
}