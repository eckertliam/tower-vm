use crate::interpreter::{
    typeflag::TypeFlag,
    instruction::Instruction,
};

// Chunk is used to abstract over the Vec<u8> that is used as code in the machine
// instead we have a vec<chunk> allowing identifiers referencing constants and accesses for collections
// the builder converts vec<chunk> to vec<u8> as its final step, but that could not be done from this level as context is needed for constants
#[derive(Debug)]
pub enum Chunk {
    Type(TypeFlag),
    Instruction(Instruction),
    Identifier(String),
    Access(String, usize),// access a collection like a[0] etc
}

impl Chunk {
    pub fn new_type(ty_flag: TypeFlag) -> Chunk {
        Chunk::Type(ty_flag)
    }

    pub fn new_instr(instr: Instruction) -> Chunk {
        Chunk::Instruction(instr)
    }

    pub fn new_ident(ident: &str) -> Chunk {
        Chunk::Identifier(ident.to_string())
    }

    pub fn new_access(ident: &str, idx: usize) -> Chunk {
        Chunk::Access(ident.to_string(), idx)
    }
}
