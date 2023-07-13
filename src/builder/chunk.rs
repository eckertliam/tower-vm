use crate::{
    Instruction,
    TypeFlag,
    Value,
    Function,
};


// Chunk is used to abstract over the Vec<u8> that is used as code in the machine
// instead we have a vec<chunk> allowing identifiers referencing constants and accesses for collections
// the builder converts vec<chunk> to vec<u8> as its final step, but that could not be done from this level as context is needed for constants
#[derive(Debug, Clone)]
pub enum Chunk {
    Type(TypeFlag),
    Instruction(Instruction),
    Value(Value),
    Identifier(String),
    Access(String, usize),// access a collection like a[0] etc
}

impl Chunk {
    pub fn new_type(ty_flag: TypeFlag) -> Self {
        Self::Type(ty_flag)
    }

    pub fn new_instr(instr: Instruction) -> Self {
        Self::Instruction(instr)
    }

    pub fn new_ident(ident: &str) -> Self {
        Self::Identifier(ident.to_string())
    }

    pub fn new_value(value: Value) -> Self {
        Self::Value(value)
    }

    pub fn new_access(ident: &str, idx: usize) -> Self {
        Self::Access(ident.to_string(), idx)
    }

    pub fn to_code(&self, func: &Function) -> Vec<u8> {
        match self {
            Self::Type(ty) => ty.to_code().to_vec(),
            Self::Instruction(instr) => vec![*instr as u8],
            Self::Value(val) => val.to_code(),
            Self::Identifier(ident) => match func.get_const(ident) {
                Some(c) => c.get_value().to_code(),
                None => panic!("Error: Fn {} attempted to access the undefined constant {}", func.get_ident(), ident)
            }
            Self::Access(ident, idx) => match func.access_const(ident, *idx) {
                Some(val) => val.to_code(),
                None => panic!("Error: Fn {} attempted to access the undefined constant {}", func.get_ident(), ident),
            }
        }
    }
}
