use crate::{
    Chunk,
    Instruction,
    Constant,
    TypeFlag,
    Value,
};

// the function serves as an abstraction for the builder
// each function is built up with its instructions, constants, and identifier
// once all functions are built an entry function is designated and all functions called by entry function are raised to top-level
// all function code fields are converted to Vec<u8> and all constants and identifiers are done away with or inlined
pub struct Function {
    ident: String,
    chunks: Vec<Chunk>,
    constants: Vec<Constant>,

    // dimensions of the function in top
    pub address: Option<usize>,
    pub len: Option<usize>,
}

impl Function {
    pub fn new(id: &str) -> Self {
        Self {
            ident: id.to_string(),
            chunks: vec![],
            constants: vec![],

            address: None,
            len: None
        }
    }

    pub fn get_ident(&self) -> &str {
        &self.ident
    }


    pub fn push_constant(&mut self, c: Constant) {
        self.constants.push(c);
    }

    // get a constant
    pub fn get_const(&self, ident: &str) -> Option<Constant> {
        for c in &self.constants {
            if c.get_ident() == ident {
                return Some(c.clone());
            }
        }
        return None;
    }

    // access an idx in a constant collection
    pub fn access_const(&self, ident: &str, idx: usize) -> Option<Value> {
        match self.get_const(ident) {
            Some(c) => Some(c.access_value(idx)),
            None => None,
        }
    }

    // convert the function to code(Vec<u8>) in order to raise up to top
    pub fn to_code(&mut self) -> Vec<u8> {
        let mut code: Vec<u8> = vec![];
        for chunk in self.chunks.clone() {
            code.append(&mut chunk.to_code(&self))
        }
        code
    }

    pub fn push_type(&mut self, ty: TypeFlag) {
        self.chunks.push(Chunk::Type(ty))
    }

    pub fn push_instr(&mut self, instr: Instruction) {
        self.chunks.push(Chunk::Instruction(instr))
    }

    pub fn push_value(&mut self, value: Value) {
        self.chunks.push(Chunk::Value(value))
    }

    pub fn push_identifier(&mut self, ident: &str) {
        self.chunks.push(Chunk::Identifier(ident.to_string()))
    }

    pub fn push_access(&mut self, ident: &str, idx: usize) {
        self.chunks.push(Chunk::Access(ident.to_string(), idx))
    }
}