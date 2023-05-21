use crate::gensym::Gensym;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    UChar,
    Char,
    UInt,
    Null,
}

impl Into<String> for Type {
    fn into(self) -> String {
        match self {
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Bool => "bool".to_string(),
            Type::UChar => "unsigned char".to_string(),
            Type::Char => "char".to_string(),
            Type::UInt => "unsigned int".to_string(),
            Type::Null => "void*".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    UChar(u8),
    Char(char),
    UInt(u32),
    Null,
    Symbol(String, Type),
}

impl Value {
    pub fn as_var(&self, id: &str) -> (String, Type) {
        match self {
            Value::Int(int) => (format!("int {} = {};", id, int), Type::Int),
            Value::Float(float) => (format!("float {} = {};", id, float), Type::Float),
            Value::Bool(b) => (format!("bool {} = {};", id, b), Type::Bool),
            Value::UChar(uchar) => (format!("unsigned char {} = {};", id, uchar), Type::UChar),
            Value::Char(c) => (format!("char {} = {};", id, c), Type::Char),
            Value::UInt(uint) => (format!("unsigned int {} = {};", id, uint), Type::UInt),
            Value::Null => (format!("void* {} = NULL;", id), Type::Null),
            Value::Symbol(symbol, t) => {
                let type_: String = t.clone().into();
                (format!("{} {} = {};", type_, id, symbol), t.clone())
            }
        }
    }

    pub fn get_type(&self) -> Type {
        match self {
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::Bool(_) => Type::Bool,
            Value::UChar(_) => Type::UChar,
            Value::Char(_) => Type::Char,
            Value::UInt(_) => Type::UInt,
            Value::Null => Type::Null,
            Value::Symbol(_, t) => t.clone(),
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Value::Symbol(_, _) => true,
            _ => false,
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            Value::Symbol(symbol, _) => symbol.clone(),
            _ => panic!("Value is not a symbol"),
        }
    }
}

impl Into<String> for Value {
    fn into(self) -> String {
        match self {
            Value::Int(int) => int.to_string(),
            Value::Float(float) => float.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::UChar(uchar) => uchar.to_string(),
            Value::Char(c) => c.to_string(),
            Value::UInt(uint) => uint.to_string(),
            Value::Null => "null".to_string(),
            Value::Symbol(symbol, _) => symbol,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let value: String = self.clone().into();
        write!(f, "{}", value)
    }
}

impl From<i32> for Value {
    fn from(int: i32) -> Self {
        Value::Int(int)
    }
}

impl From<f32> for Value {
    fn from(float: f32) -> Self {
        Value::Float(float)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<u8> for Value {
    fn from(uchar: u8) -> Self {
        Value::UChar(uchar)
    }
}

impl From<char> for Value {
    fn from(c: char) -> Self {
        Value::Char(c)
    }
}

impl From<u32> for Value {
    fn from(uint: u32) -> Self {
        Value::UInt(uint)
    }
}
