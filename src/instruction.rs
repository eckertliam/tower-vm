#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // integer operations
    I_LOAD(i32),
    I_ADD,
    I_SUB,
    I_MUL,
    I_DIV,
    I_MOD,
    I_SHL,
    I_SHR,
    I_AND,
    I_OR,
    I_XOR,
    I_EQ,
    I_NE,
    I_LT,
    I_LE,
    I_GT,
    I_GE,
    I_PRINT,

    
    // floating point operations
    F_LOAD(f32),
    F_ADD,
    F_SUB,
    F_MUL,
    F_DIV,
    F_MOD,
    F_EQ,
    F_NE,
    F_LT,
    F_LE,
    F_GT,
    F_GE,
    F_PRINT,


    // boolean operations
    B_LOAD(bool),
    B_NOT,
    B_AND,
    B_OR,
    B_EQ,
    B_NE,
    B_PRINT,
    B_IF(Vec<Instruction>),
}
