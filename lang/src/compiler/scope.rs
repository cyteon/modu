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

}

pub enum Variable {
    Local(usize),
    Global(usize),
}