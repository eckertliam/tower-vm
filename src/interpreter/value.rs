#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeFlag {
    I8 = 0,
    I16 = 1,
    I32 = 2,
    I64 = 3,
    F32 = 4,
    F64 = 5,
    Bool = 6,
    Char = 7,
    U8 = 8,
    U16 = 9,
    U32 = 10,
    U64 = 11,
}

impl TypeFlag {
    pub fn to_code(self) -> [u8; 2] {
        // first byte is instruction TYPE, second byte is the type flag
        [1, self.into()]
    }
}

impl Into<u8> for TypeFlag {
    fn into(self) -> u8 {
        self as u8
    }
}

impl Into<u64> for TypeFlag {
    fn into(self) -> u64 {
        self as u64
    }
}

impl From<u8> for TypeFlag {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Self::I8,
            1 => Self::I16,
            2 => Self::I32,
            3 => Self::I64,
            4 => Self::F32,
            5 => Self::F64,
            6 => Self::Bool,
            7 => Self::Char,
            8 => Self::U8,
            9 => Self::U16,
            10 => Self::U32,
            11 => Self::U64,
            _ => panic!("invalid type flag"),
        }
    }
}

impl TypeFlag {
    pub fn size(self) -> usize {
        match self {
            Self::I8 => 1,
            Self::I16 => 2,
            Self::I32 => 4,
            Self::I64 => 8,
            Self::F32 => 4,
            Self::F64 => 8,
            Self::Bool => 1,
            Self::Char => 4,
            Self::U8 => 1,
            Self::U16 => 2,
            Self::U32 => 4,
            Self::U64 => 8,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
    pub ty: TypeFlag,
    pub data: u64,
}

impl Value {
    pub fn to_code(self) -> Vec<u8> {
        let mut code = Vec::new();
        code.extend_from_slice(&self.ty.to_code());
        code.extend_from_slice(&self.data.to_le_bytes());
        code
    }

    pub fn from_code(ty: TypeFlag, code: &[u8]) -> Self {
        let mut data = [0; 8];
        data.copy_from_slice(&code[..8]);
        Self {
            ty,
            data: u64::from_le_bytes(data),
        }
    }

    pub fn to_stack(self) -> u64 {
        self.data
    }

    pub fn from_stack(ty: TypeFlag, stack: u64) -> Self {
        Self { ty, data: stack }
    }
}

macro_rules! impl_from_value {
    ($from_type:ty, $type_flag:ident) => {
        impl From<$from_type> for Value {
            fn from(value: $from_type) -> Self {
                Self {
                    ty: TypeFlag::$type_flag,
                    data: value as u64,
                }
            }
        }
    };
}

impl_from_value!(i8, I8);
impl_from_value!(i16, I16);
impl_from_value!(i32, I32);
impl_from_value!(i64, I64);

impl_from_value!(char, Char);
impl_from_value!(bool, Bool);

impl_from_value!(u8, U8);
impl_from_value!(u16, U16);
impl_from_value!(u32, U32);
impl_from_value!(u64, U64);

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self {
            ty: TypeFlag::F32,
            data: value.to_bits() as u64,
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self {
            ty: TypeFlag::F64,
            data: value.to_bits(),
        }
    }
}

macro_rules! impl_try_into_value {
    ($to_type:ty, $type_flag:ident) => {
        impl TryInto<$to_type> for Value {
            type Error = &'static str;

            fn try_into(self) -> Result<$to_type, Self::Error> {
                if self.ty == TypeFlag::$type_flag {
                    Ok(self.data as $to_type)
                } else {
                    Err("invalid type")
                }
            }
        }
    };
}

impl_try_into_value!(i8, I8);
impl_try_into_value!(i16, I16);
impl_try_into_value!(i32, I32);
impl_try_into_value!(i64, I64);

impl TryInto<char> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<char, Self::Error> {
        if self.ty == TypeFlag::Char {
            Ok(char::from_u32(self.data as u32).unwrap())
        } else {
            Err("invalid type")
        }
    }
}

impl TryInto<bool> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<bool, Self::Error> {
        if self.ty == TypeFlag::Bool {
            Ok(self.data != 0)
        } else {
            Err("invalid type")
        }
    }
}

impl_try_into_value!(u8, U8);
impl_try_into_value!(u16, U16);
impl_try_into_value!(u32, U32);
impl_try_into_value!(u64, U64);

impl TryInto<f32> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<f32, Self::Error> {
        if self.ty == TypeFlag::F32 {
            Ok(f32::from_bits(self.data as u32))
        } else {
            Err("invalid type")
        }
    }
}

impl TryInto<f64> for Value {
    type Error = &'static str;

    fn try_into(self) -> Result<f64, Self::Error> {
        if self.ty == TypeFlag::F64 {
            Ok(f64::from_bits(self.data))
        } else {
            Err("invalid type")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // value lifecycle tests
    // STEPS:
    // 1. init primitive value ie. i8, i16, i32, i64, f32, f64, bool, char, u8, u16, u32, u64
    // 2. convert to value
    // 3. convert to [u8] using to_code() (this is used when values are inlined in the bytecode)
    // 4. convert to value using from_code() (this is used when values are take from bytecode then pushed to the stack)
    // 5. convert to u64 using to_stack() (this is used when values are pushed to the stack)
    // 6. convert back to value with from_stack() and a type flag (this is used when values are popped from the stack)
    // 7. convert back to primitive value
    // 8. assert that the primitive value is equal to the initial value
    // lossless conversions.

    #[test]
    fn test_i8_lifecycle() {
        let init: i8 = 42;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..]; // drop the type flag
        let decode = Value::from_code(TypeFlag::I8, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::I8, stack_push);
        let prim_test: i8 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_i16_lifecycle() {
        let init: i16 = -1000;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::I16, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::I16, stack_push);
        let prim_test: i16 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_i32_lifecycle() {
        let init: i32 = 123456789;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::I32, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::I32, stack_push);
        let prim_test: i32 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_i64_lifecycle() {
        let init: i64 = -9876543210;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::I64, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::I64, stack_push);
        let prim_test: i64 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_f32_lifecycle() {
        let init: f32 = std::f32::consts::PI;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::F32, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::F32, stack_push);
        let prim_test: f32 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_f64_lifecycle() {
        let init: f64 = std::f64::consts::PI;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::F64, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::F64, stack_push);
        let prim_test: f64 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_true_lifecycle() {
        let init: bool = true;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::Bool, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::Bool, stack_push);
        let prim_test: bool = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_false_lifecycle() {
        let init: bool = false;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::Bool, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::Bool, stack_push);
        let prim_test: bool = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_char_lifecycle() {
        let init: char = 'A';
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::Char, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::Char, stack_push);
        let prim_test: char = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_u8_lifecycle() {
        let init: u8 = 255;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::U8, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::U8, stack_push);
        let prim_test: u8 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_u16_lifecycle() {
        let init: u16 = 65535;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::U16, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::U16, stack_push);
        let prim_test: u16 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_u32_lifecycle() {
        let init: u32 = 4294967295;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::U32, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::U32, stack_push);
        let prim_test: u32 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }

    #[test]
    fn test_u64_lifecycle() {
        let init: u64 = 18446744073709551615;
        let value = Value::from(init);
        let code: &[u8] = &value.to_code()[2..];
        let decode = Value::from_code(TypeFlag::U64, code);
        let stack_push = decode.to_stack();
        let stack_pop = Value::from_stack(TypeFlag::U64, stack_push);
        let prim_test: u64 = stack_pop.try_into().unwrap();
        assert_eq!(prim_test, init);
    }
}
