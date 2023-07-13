use crate::{
    Chunk,
    Instruction,
    Constant,
    TypeFlag
};

// the function serves as an abstraction for the builder
// each function is built up with its instructions, constants, and identifier
// once all functions are built an entry function is designated and all functions called by entry function are raised to top-level
// all function code fields are converted to Vec<u8> and all constants and identifiers are done away with or inlined
pub struct Function {
    ident: String,
    code: Vec<Chunk>,
    address: Option<usize>,
    constants: Vec<Constant>,
}

