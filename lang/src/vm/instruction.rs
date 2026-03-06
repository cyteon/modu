#[repr(u8)]
#[derive(Debug)]
pub enum Instruction {
    Push(usize),
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
    IndexGet,
    IndexSet,
}