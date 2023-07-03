use std::mem::transmute;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value {
    I8(i8),
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
    pub fn to_i8(raw: u64) -> Value {
        Value::I8(raw as i8)
    }

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
        unsafe {
            Value::F32(transmute::<u32, f32>(raw as u32))
        
        }
    }

    pub fn to_f64(raw: u64) -> Value {
        unsafe {
            Value::F64(transmute::<u64, f64>(raw))
        }
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
            Value::I8(v) => v as u64,
            Value::I16(v) => v as u64,
            Value::I32(v) => v as u64,
            Value::I64(v) => v as u64,
            Value::F32(v) => {
                unsafe {
                    transmute::<f32, u32>(v) as u64
                }
            }
            Value::F64(v) => {
                unsafe {
                    transmute::<f64, u64>(v)
                }
            }
            Value::Bool(v) => v as u64,
            Value::Char(v) => v as u64,
            Value::U8(v) => v as u64,
            Value::U16(v) => v as u64,
            Value::U32(v) => v as u64,
            Value::U64(v) => v,
        }
    }
}

macro_rules! impl_try_into {
    ($type:ty, $variant:ident) => {
        impl TryInto<$type> for Value {
            type Error = String;

            fn try_into(self) -> Result<$type, Self::Error> {
                match self {
                    Value::$variant(v) => Ok(v),
                    _ => Err(format!("Cannot convert {:?} to {}", self, stringify!($type))),
                }
            }
        }
    };
}

impl_try_into!(i8, I8);
impl_try_into!(i16, I16);
impl_try_into!(i32, I32);
impl_try_into!(i64, I64);
impl_try_into!(f32, F32);
impl_try_into!(f64, F64);
impl_try_into!(bool, Bool);
impl_try_into!(char, Char);
impl_try_into!(u8, U8);
impl_try_into!(u16, U16);
impl_try_into!(u32, U32);

macro_rules! impl_from {
    ($type:ty, $variant:ident) => {
        impl From<$type> for Value {

            fn from(value: $type) -> Self {
                Value::$variant(value)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(f32, F32);
impl_from!(f64, F64);
impl_from!(bool, Bool);
impl_from!(char, Char);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);




#[cfg(test)]
mod tests {
    use super::*;

    // each of these round tests takes a value through the following steps:
    // 1. create a Value from the initial value
    // 2. convert the Value into a u64
    // 3. convert the u64 back into a Value
    // 4. convert the Value into the initial value type
    // 5. assert that the initial value and the final value are equal
    // this verifies lossless round-tripping through the lifecycle of a Value
    #[test]
    fn test_i8_round() {
        let init: i8 = 22;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_i8(encoded);
        let out: i8 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_i16_round() {
        let init: i16 = 420;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_i16(encoded);
        let out: i16 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_i32_round() {
        let init: i32 = 20000;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_i32(encoded);
        let out: i32 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_i64_round() {
        let init: i64 = 2020100000000000;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_i64(encoded);
        let out: i64 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_f32_round() {
        let init: f32 = std::f32::consts::PI;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_f32(encoded);
        let out: f32 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_f64_round() {
        let init: f64 = std::f64::consts::PI;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_f64(encoded);
        let out: f64 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_true_round() {
        let init: bool = true;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_bool(encoded);
        let out: bool = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_false_round() {
        let init: bool = false;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_bool(encoded);
        let out: bool = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_char_round() {
        let init: char = 'a';
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_char(encoded);
        let out: char = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_u8_round() {
        let init: u8 = 22;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_u8(encoded);
        let out: u8 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_u16_round() {
        let init: u16 = 420;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_u16(encoded);
        let out: u16 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_u32_round() {
        let init: u32 = 20000;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_u32(encoded);
        let out: u32 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }

    #[test]
    fn test_u64_round() {
        let init: u64 = 2020100000000000;
        let in_val: Value = init.into();
        let encoded: u64 = in_val.into();
        let decoded: Value = Value::to_u64(encoded);
        let out: u64 = decoded.try_into().unwrap();
        assert_eq!(init, out);
    }
}