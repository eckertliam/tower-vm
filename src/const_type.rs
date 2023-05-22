#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConstType {
    I32,
    F32,
    Bool,
}

impl ConstType {
    pub fn to_string(&self) -> String {
        match self {
            ConstType::I32 => "int".to_string(),
            ConstType::F32 => "float".to_string(),
            ConstType::Bool => "bool".to_string(),
        }
    }
}
