use crate::{Value, TypeFlag, Instruction};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Chunk {
    Literal(Value),// expected after push
    Type(TypeFlag), // expected after SetType 
    Instr(Instruction),    
}

impl Chunk {
    pub fn to_code(&mut self) -> Vec<u8> {
        use Chunk::*;

        match self {
            Literal(value) => value.to_code(),
            Type(ty_flag) => ty_flag.to_code(),
            Instr(instr) => vec![*instr as u8]
        }
    }
}
