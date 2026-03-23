use std::collections::HashMap;

pub struct Scope {
    locals: HashMap<String, (usize, bool)>, // slot, is_const
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
    next_slot: usize,
    pub max_slot: usize,
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

    pub fn enter_function(&mut self) -> (usize, usize) {
        self.function_depth += 1;

        let saved_next = self.next_slot;
        let saved_max = self.max_slot;

        self.next_slot = 0;
        self.max_slot = 0;

        self.push_scope();

        (saved_next, saved_max)
    }

    pub fn exit_function(&mut self, saved: (usize, usize)) -> usize {
        let locals_count = self.max_slot;

        self.function_depth -= 1;
        
        self.pop_scope();
        self.next_slot = saved.0;
        self.max_slot = saved.1;

        locals_count
    }

    pub fn resolve(&self, name: &str) -> Variable {
        for scope in self.scopes.iter().rev() {
            if let Some(&(slot, _)) = scope.locals.get(name) {
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
            scope.locals.insert(name.to_string(), (slot, false));
        }

        slot
    }

    pub fn define_const(&mut self, name: &str) -> usize {
        let slot = self.next_slot;
        self.next_slot += 1;

        if self.next_slot > self.max_slot {
            self.max_slot = self.next_slot;
        }

        if let Some(scope) = self.scopes.last_mut() {
            scope.locals.insert(name.to_string(), (slot, true));
        }

        slot
    }

    pub fn is_const(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if let Some(&(_, c)) = scope.locals.get(name) {
                return c;
            }
        }
        
        false
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(Scope {
            locals: HashMap::new(),
        });
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }
}