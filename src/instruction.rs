#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    // integer operations
    ILoad(i32),
    IAdd,
    ISub,
    IMul,
    IDiv,
    IMod,
    IShl,
    IShr,
    IAnd,
    IOr,
    IXor,
    IEq,
    INeq,
    ILt,
    ILte,
    IGt,
    IGte,
    IPrint,

    
    // floating point operations
    FLoad(f32),
    FAdd,
    FSub,
    FMul,
    FDiv,
    FMod,
    FEq,
    FNeq,
    FLt,
    FLte,
    FGt,
    FGte,
    FPrint,


    // boolean operations
    BLoad(bool),
    BNot,
    BAnd,
    BOr,
    BEq,
    BNeq,
    BPrint,
    BIf(Vec<Instruction>),
}
