use crate::{
    gensym::Gensym, const_type::ConstType,
};


pub struct ConstantTable {
    types: Vec<ConstType>,
    id: Vec<String>,
    gensym: Gensym,
}

impl ConstantTable {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            id: Vec::new(),
            gensym: Gensym::new(),
        }
    }

    pub fn push(&mut self, constant: ConstType) -> String {
        let symbol = self.gensym.next();
        self.types.push(constant);
        self.id.push(symbol.clone());
        symbol
    }

    pub fn get(&self, symbol: &str) -> Option<&ConstType> {
        let index = self.id.iter().position(|s| s == symbol)?;
        self.types.get(index)
    }
}