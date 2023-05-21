use crate::{
    gensym::Gensym,
    constant::Constant,
};


pub struct ConstantTable {
    constants: Vec<Constant>,
    id: Vec<String>,
    gensym: Gensym,
}

impl ConstantTable {
    pub fn new() -> Self {
        Self {
            constants: Vec::new(),
            id: Vec::new(),
            gensym: Gensym::new(),
        }
    }

    pub fn push(&mut self, constant: Constant) -> String {
        let symbol = self.gensym.next();
        self.constants.push(constant);
        self.id.push(symbol.clone());
        symbol
    }

    pub fn get(&self, symbol: &str) -> Option<&Constant> {
        let index = self.id.iter().position(|s| s == symbol)?;
        self.constants.get(index)
    }

    pub fn drop(&mut self, symbol: &str) -> Option<Constant> {
        let index = self.id.iter().position(|s| s == symbol)?;
        self.id.remove(index);
        Some(self.constants.remove(index))
    }
}