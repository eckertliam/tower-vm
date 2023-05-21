#[derive(Clone)]
pub enum Constant {
    I32(i32),
    F32(f32),
    Bool(bool),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    I32,
    F32,
    Bool,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::I32 => "int".to_string(),
            Type::F32 => "float".to_string(),
            Type::Bool => "bool".to_string(),
        }
    }
}

impl Constant {
    pub fn to_string(&self) -> String {
        match self {
            Constant::I32(i) => i.to_string(),
            Constant::F32(f) => f.to_string(),
            Constant::Bool(b) => b.to_string(),
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Constant::I32(_) => Type::I32,
            Constant::F32(_) => Type::F32,
            Constant::Bool(_) => Type::Bool,
        }
    }
}

impl From<i32> for Constant {
    fn from(i: i32) -> Self {
        Self::I32(i)
    }
}

impl From<f32> for Constant {
    fn from(f: f32) -> Self {
        Self::F32(f)
    }
}

impl From<bool> for Constant {
    fn from(b: bool) -> Self {
        Self::Bool(b)
    }
}