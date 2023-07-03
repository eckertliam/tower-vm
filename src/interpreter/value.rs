#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Bool(bool),
    Char(char),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl Value {
    pub fn to_i16(raw: u64) -> Value {
        Value::I16(raw as i16)
    }

    pub fn to_i32(raw: u64) -> Value {
        Value::I32(raw as i32)
    }

    pub fn to_i64(raw: u64) -> Value {
        Value::I64(raw as i64)
    }

    pub fn to_f32(raw: u64) -> Value {
        Value::F32(raw as f32)
    }

    pub fn to_f64(raw: u64) -> Value {
        Value::F64(raw as f64)
    }

    pub fn to_bool(raw: u64) -> Value {
        Value::Bool(raw != 0)
    }

    pub fn to_char(raw: u64) -> Value {
        Value::Char(raw as u8 as char)
    }

    pub fn to_u8(raw: u64) -> Value {
        Value::U8(raw as u8)
    }

    pub fn to_u16(raw: u64) -> Value {
        Value::U16(raw as u16)
    }

    pub fn to_u32(raw: u64) -> Value {
        Value::U32(raw as u32)
    }

    pub fn to_u64(raw: u64) -> Value {
        Value::U64(raw)
    }
}

impl Into<u64> for Value {
    fn into(self) -> u64 {
        match self {
            Value::I16(v) => v as u64,
            Value::I32(v) => v as u64,
            Value::I64(v) => v as u64,
            Value::F32(v) => v as u64,
            Value::F64(v) => v as u64,
            Value::Bool(v) => v as u64,
            Value::Char(v) => v as u64,
            Value::U8(v) => v as u64,
            Value::U16(v) => v as u64,
            Value::U32(v) => v as u64,
            Value::U64(v) => v,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_i16() {
        let n: u64 = 100;
        let v = Value::to_i16(n);
        assert_eq!(v, Value::I16(100));
    }

    #[test]
    fn test_to_i32() {
        let n: u64 = 105;
        let v = Value::to_i32(n);
        assert_eq!(v, Value::I32(105));
    }

    #[test]
    fn test_to_i64() {
        let n: u64 = 110;
        let v = Value::to_i64(n);
        assert_eq!(v, Value::I64(110));
    }
}