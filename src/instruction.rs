use crate::value::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Const(Value),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Lt,
    Gt,
    Leq,
    Geq,
    And,
    Or,
    Not,
    Neg,
    Print,
    Pop,
    Xor,
    Shl,
    Shr,
    Return,
    Call,
    If,
    Read,
}
