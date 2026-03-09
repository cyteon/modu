#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Instruction {
    Push(usize),
    PushNull,
    Pop,

    Add, Sub, Mul, Div, Mod, Pow, Neg,
    Eq, Neq, Gt, Lt, Gte, Lte,
    In, NotIn,

    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(String),
    StoreGlobal(String),

    Jump(usize),
    JumpIfFalse(usize),

    Call(usize),
    CallMethod { argc: usize, target_local: Option<usize>, target_global: Option<String> },
    Return,

    MakeArray(usize),
    MakeObject(usize),
    MakeRange { inclusive: bool },

    GetProperty(String),
    SetProperty(String),
    IndexGet,
    IndexSet,

    IterNext {
        slot_iter: usize,
        slot_index: usize,
        slot_var: usize,
    },

    Import {
        path: String,
        alias: Option<String>,
    }
}