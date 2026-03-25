#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Instruction {
    Push(usize),
    PushNull,
    Pop,
    Swap,
    Rotate3,

    BitAnd, BitOr, BitXor, BitShl, BitShr, BitNot,
    Add, Sub, Mul, Div, Mod, Pow, Neg,
    Eq, Neq, Gt, Lt, Gte, Lte,
    In, NotIn, Not,

    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(String),
    StoreGlobal(String),

    Jump(usize),
    JumpIfFalse(usize),

    SetupTry(usize), // jumps to usize if smth went wrong
    EndTry,

    Call(usize),
    CallMethod { argc: usize, target_local: Option<usize>, target_global: Option<String> },
    Return,

    MakeArray(usize),
    MakeObject(usize),
    MakeRange { inclusive: bool },

    Extend,
    GetSuper(String),

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