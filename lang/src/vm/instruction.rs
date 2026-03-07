#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    Push(usize),
    PushNull,
    Pop,

    Add, Sub, Mul, Div, Mod, Pow, Neg,
    Eq, Neq, Gt, Lt, Gte, Lte,
    Not,

    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(String),
    StoreGlobal(String),

    Jump(usize),
    JumpIfFalse(usize),
    JumpIfTrue(usize),

    Call(usize),
    Return,

    MakeArray(usize),
    MakeRange { inclusive: bool },

    IndexGet,
    IndexSet,

    IterNext {
        slot_iter: usize,
        slot_index: usize,
        slot_var: usize,
    }
}