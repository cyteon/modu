use std::collections::HashMap;

pub struct Scope {
    locals: HashMap<String, usize>,
    depth: usize,
}

pub struct ScopeStack {
    scopes: Vec<Scope>,
    next_slot: usize,
    globals: HashMap<String, usize>,
    next_global: usize,
}


impl ScopeStack {
    pub fn new() -> Self {
        Self {
            scopes: Vec::new(),
            next_slot: 0,
            globals: HashMap::new(),
            next_global: 0,
        }
    }
}

pub enum Variable {
    Local(usize),
    Global(usize),
}