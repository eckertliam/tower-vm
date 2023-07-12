use crate::interpreter::typeflag::TypeFlag;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Value {
    pub ty: TypeFlag,
    pub data: u64,
}

impl Value {
    pub fn to_code(self) -> Vec<u8> {
        // trim data bytes to type size
        self.data.to_le_bytes()[..self.ty.size()].to_vec()
    }

    pub fn from_code(ty: TypeFlag, code: &[u8]) -> Self {
        let mut data = [0; 8];
        if code.len() != ty.size() {
            panic!("{:?} expects {} bytes, got {}", ty, ty.size(), code.len());
        }else if code.len() != 8 {
            let mut idx = 0;
            for byte in code {
                data[idx] = *byte;
                idx += 1;
            }
        }else{
            data.copy_from_slice(code);
        }
        Self {
            ty,
            data: u64::from_le_bytes(data),
        }
    }

    pub fn to_stack(self) -> u64 {
        self.data
    }

    pub fn from_stack(ty: TypeFlag, raw: u64) -> Self {
        Self { ty, data: raw }
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

        impl TryInto<$to_type> for &Value {
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

impl TryInto<char> for &Value {
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

impl TryInto<bool> for &Value {
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

impl TryInto<f32> for &Value {
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

impl TryInto<f64> for &Value {
    type Error = &'static str;

    fn try_into(self) -> Result<f64, Self::Error> {
        if self.ty == TypeFlag::F64 {
            Ok(f64::from_bits(self.data))
        } else {
            Err("invalid type")
        }
    }
}

macro_rules! impl_arith_ops {
    ($trait:path, $fn_name:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Self;

                fn $fn_name(self, rhs: Self) -> Self::Output {
                    use TypeFlag::*;
                    match (self.ty, rhs.ty) {
                        (I8, I8) => {
                            let lhs: i8 = self.try_into().unwrap();
                            let rhs: i8 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (I16, I16) => {
                            let lhs: i16 = self.try_into().unwrap();
                            let rhs: i16 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (I32, I32) => {
                            let lhs: i32 = self.try_into().unwrap();
                            let rhs: i32 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (I64, I64) => {
                            let lhs: i64 = self.try_into().unwrap();
                            let rhs: i64 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (F32, F32) => {
                            let lhs: f32 = self.try_into().unwrap();
                            let rhs: f32 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (F64, F64) => {
                            let lhs: f64 = self.try_into().unwrap();
                            let rhs: f64 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (U8, U8) => {
                            let lhs: u8 = self.try_into().unwrap();
                            let rhs: u8 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (U16, U16) => {
                            let lhs: u16 = self.try_into().unwrap();
                            let rhs: u16 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (U32, U32) => {
                            let lhs: u32 = self.try_into().unwrap();
                            let rhs: u32 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        (U64, U64) => {
                            let lhs: u64 = self.try_into().unwrap();
                            let rhs: u64 = rhs.try_into().unwrap();
                            (lhs $op rhs).into()
                        }
                        _ => panic!("invalid types"),
                    }
                }
            }
    };
}

impl_arith_ops!(std::ops::Add, add, +);
impl_arith_ops!(std::ops::Sub, sub, -);
impl_arith_ops!(std::ops::Mul, mul, *);
impl_arith_ops!(std::ops::Div, div, /);
impl_arith_ops!(std::ops::Rem, rem, %);

macro_rules! impl_bit_ops {
    ($trait:path, $fn_name:ident, $op:tt) => {
        impl $trait for Value {
            type Output = Self;

            fn $fn_name(self, rhs: Self) -> Self::Output {
                use TypeFlag::*;

                match (self.ty, rhs.ty) {
                    (I8, I8) => {
                        let lhs: i8 = self.try_into().unwrap();
                        let rhs: i8 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (I16, I16) => {
                        let lhs: i16 = self.try_into().unwrap();
                        let rhs: i16 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (I32, I32) => {
                        let lhs: i32 = self.try_into().unwrap();
                        let rhs: i32 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (I64, I64) => {
                        let lhs: i64 = self.try_into().unwrap();
                        let rhs: i64 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (U8, U8) => {
                        let lhs: u8 = self.try_into().unwrap();
                        let rhs: u8 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (U16, U16) => {
                        let lhs: u16 = self.try_into().unwrap();
                        let rhs: u16 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (U32, U32) => {
                        let lhs: u32 = self.try_into().unwrap();
                        let rhs: u32 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    (U64, U64) => {
                        let lhs: u64 = self.try_into().unwrap();
                        let rhs: u64 = rhs.try_into().unwrap();
                        (lhs $op rhs).into()
                    }
                    _ => panic!("invalid types"),
                }
            }
        }
    };
}

impl_bit_ops!(std::ops::BitAnd, bitand, &);
impl_bit_ops!(std::ops::BitOr, bitor, |);
impl_bit_ops!(std::ops::BitXor, bitxor, ^);
impl_bit_ops!(std::ops::Shl, shl, <<);
impl_bit_ops!(std::ops::Shr, shr, >>);

impl std::ops::Not for Value {
    type Output = Self;

    fn not(self) -> Self::Output {
        use TypeFlag::*;

        match self.ty {
            I8 => {
                let lhs: i8 = self.try_into().unwrap();
                (!lhs).into()
            }
            I16 => {
                let lhs: i16 = self.try_into().unwrap();
                (!lhs).into()
            }
            I32 => {
                let lhs: i32 = self.try_into().unwrap();
                (!lhs).into()
            }
            I64 => {
                let lhs: i64 = self.try_into().unwrap();
                (!lhs).into()
            }
            U8 => {
                let lhs: u8 = self.try_into().unwrap();
                (!lhs).into()
            }
            U16 => {
                let lhs: u16 = self.try_into().unwrap();
                (!lhs).into()
            }
            U32 => {
                let lhs: u32 = self.try_into().unwrap();
                (!lhs).into()
            }
            U64 => {
                let lhs: u64 = self.try_into().unwrap();
                (!lhs).into()
            }
            Bool => {
                let lhs: bool = self.try_into().unwrap();
                (!lhs).into()
            }
            _ => panic!("invalid types"),
        }
    }
}

impl std::ops::Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use TypeFlag::*;

        match self.ty {
            I8 => {
                let lhs: i8 = self.try_into().unwrap();
                (-lhs).into()
            }
            I16 => {
                let lhs: i16 = self.try_into().unwrap();
                (-lhs).into()
            }
            I32 => {
                let lhs: i32 = self.try_into().unwrap();
                (-lhs).into()
            }
            I64 => {
                let lhs: i64 = self.try_into().unwrap();
                (-lhs).into()
            }
            F32 => {
                let lhs: f32 = self.try_into().unwrap();
                (-lhs).into()
            }
            F64 => {
                let lhs: f64 = self.try_into().unwrap();
                (-lhs).into()
            }
            _ => panic!("invalid types"),
        }
    }
}

impl std::cmp::PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        if self.ty == other.ty {
            return self.data == other.data;
        }
        return false;
    }

    fn ne(&self, other: &Self) -> bool {
        if self.ty == other.ty {
            return self.data != other.data;
        }
        return true;
    }
}

impl std::cmp::PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.ty == other.ty {
            return self.data.partial_cmp(&other.data);
        }
        return None;
    }
}


impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TypeFlag::*;

        match self.ty {
            I8 => {
                let val: i8 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            I16 => {
                let val: i16 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            I32 => {
                let val: i32 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            I64 => {
                let val: i64 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            U8 => {
                let val: u8 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            U16 => {
                let val: u16 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            U32 => {
                let val: u32 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            U64 => {
                let val: u64 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            F32 => {
                let val: f32 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            F64 => {
                let val: f64 = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            Bool => {
                let val: bool = self.try_into().unwrap();
                write!(f, "{}", val)
            },
            Char => {
                let val: char = self.try_into().unwrap();
                write!(f, "{}", val)
            },
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

    macro_rules! lifecycle_test {
        ($type:ty, $flag:expr, $init:expr) => {
            let init: $type = $init;
            let value = Value::from(init);
            let code: &[u8] = &value.to_code();
            let decode = Value::from_code($flag, code);
            let stack_push = decode.to_stack();
            let stack_pop = Value::from_stack($flag, stack_push);
            let prim_test: $type = stack_pop.try_into().unwrap();
            assert_eq!(prim_test, init);
        };
    }

    #[test]
    fn test_i8_lifecycle() {
        lifecycle_test!(i8, TypeFlag::I8, -100);
        lifecycle_test!(i8, TypeFlag::I8, 126);
    }

    #[test]
    fn test_i16_lifecycle() {
        lifecycle_test!(i16, TypeFlag::I16, -20);
        lifecycle_test!(i16, TypeFlag::I16, 12345);
    }

    #[test]
    fn test_i32_lifecycle() {
        lifecycle_test!(i32, TypeFlag::I32, 7895);
        lifecycle_test!(i32, TypeFlag::I32, 1234567890);
    }

    #[test]
    fn test_i64_lifecycle() {
        lifecycle_test!(i64, TypeFlag::I64, 1234567890);
        lifecycle_test!(i64, TypeFlag::I64, 1234567890123456789);
    }

    #[test]
    fn test_f32_lifecycle() {
        lifecycle_test!(f32, TypeFlag::F32, std::f32::consts::PI);
        lifecycle_test!(f32, TypeFlag::F32, std::f32::consts::E);
    }

    #[test]
    fn test_f64_lifecycle() {
        lifecycle_test!(f64, TypeFlag::F64, std::f64::consts::PI);
        lifecycle_test!(f64, TypeFlag::F64, std::f64::consts::E);
    }

    #[test]
    fn test_bool_lifecycle() {
        lifecycle_test!(bool, TypeFlag::Bool, true);
        lifecycle_test!(bool, TypeFlag::Bool, false);
    }

    #[test]
    fn test_char_lifecycle() {
        lifecycle_test!(char, TypeFlag::Char, 'a');
        lifecycle_test!(char, TypeFlag::Char, 'b');
        lifecycle_test!(char, TypeFlag::Char, 'c');
    }

    #[test]
    fn test_u8_lifecycle() {
        lifecycle_test!(u8, TypeFlag::U8, 255);
        lifecycle_test!(u8, TypeFlag::U8, 0);
        lifecycle_test!(u8, TypeFlag::U8, 127);
    }

    #[test]
    fn test_u16_lifecycle() {
        lifecycle_test!(u16, TypeFlag::U16, 65535);
        lifecycle_test!(u16, TypeFlag::U16, 0);
        lifecycle_test!(u16, TypeFlag::U16, 32767);
    }

    #[test]
    fn test_u32_lifecycle() {
        lifecycle_test!(u32, TypeFlag::U32, 4294967295);
        lifecycle_test!(u32, TypeFlag::U32, 0);
        lifecycle_test!(u32, TypeFlag::U32, 2147483647);
    }

    #[test]
    fn test_u64_lifecycle() {
        lifecycle_test!(u64, TypeFlag::U64, 18446744073709551615);
        lifecycle_test!(u64, TypeFlag::U64, 0);
        lifecycle_test!(u64, TypeFlag::U64, 9223372036854775807);
    }

    // std::ops tests
    // add steps:
    // 1. create two values lhs and rhs
    // 2. create a value called expected that is the known result of lhs + rhs
    // 3. create a value called actual that is lhs + rhs
    // 4. assert that expected == actual
    macro_rules! bin_test {
        ($type:ty, $lhs:expr, $rhs:expr, $op:tt) => {
            let lhs = Value::from($lhs);
            let rhs = Value::from($rhs);
            let expected = Value::from($lhs $op $rhs);
            let actual = lhs $op rhs;
            assert_eq!(expected, actual);
        };
    }

    macro_rules! all_arith_tests {
        ($type:ty, $lhs:expr, $rhs:expr) => {
            bin_test!($type, $lhs, $rhs, +);

            bin_test!($type, $lhs, $rhs, -);

            bin_test!($type, $lhs, $rhs, *);

            bin_test!($type, $lhs, $rhs, /);

            bin_test!($type, $lhs, $rhs, %);
        };
    }

    #[test]
    fn test_i8_arith() {
        all_arith_tests!(i8, 2, 5);
        all_arith_tests!(i8, 100, 1);
        all_arith_tests!(i8, 20, 4);
    }

    #[test]
    fn test_i16_artih() {
        all_arith_tests!(i16, 20, 5);
        all_arith_tests!(i16, 100, 20);
        all_arith_tests!(i16, 3, 4);
    }

    #[test]
    fn test_i32_arith() {
        all_arith_tests!(i32, 88, 44);
        all_arith_tests!(i32, 56, 8);
        all_arith_tests!(i32, 100, 5);
    }

    #[test]
    fn test_i64_arith() {
        all_arith_tests!(i64, 100, 5);
        all_arith_tests!(i64, 648, 111);
        all_arith_tests!(i64, 86, 4);
    }

    #[test]
    fn test_f32_arith() {
        all_arith_tests!(f32, std::f32::consts::PI, 1.3456f32);
        all_arith_tests!(f32, std::f32::consts::E, 13.7689f32);
        all_arith_tests!(f32, 1.067f32, 1.12310f32);

    }

    #[test]
    fn test_f64_arith() {
        all_arith_tests!(f64, std::f64::consts::PI, 1.3456f64);
        all_arith_tests!(f64, std::f64::consts::E, 13.7689f64);
        all_arith_tests!(f64, 1.067f64, 1.12310f64);
    }


    #[test]
    fn test_u8_arith() {
        all_arith_tests!(u8, 100, 5);
        all_arith_tests!(u8, 255, 1);
        all_arith_tests!(u8, 86, 4);
    }

    #[test]
    fn test_u16_arith() {
        all_arith_tests!(u16, 100, 5);
        all_arith_tests!(u16, 65535, 1);
        all_arith_tests!(u16, 86, 4);
    }

    #[test]
    fn test_u32_arith() {
        all_arith_tests!(u32, 100, 5);
        all_arith_tests!(u32, 586858, 1);
        all_arith_tests!(u32, 86, 4);
    }

    #[test]
    fn test_u64_arith() {
        all_arith_tests!(u64, 100, 5);
        all_arith_tests!(u64, 586858, 1);
        all_arith_tests!(u64, 86, 4);
    }

    // std::ops::BitAnd tests

    macro_rules! all_bit_tests {
        ($type:ty, $lhs:expr, $rhs:expr) => {
            bin_test!($type, $lhs, $rhs, &);
            bin_test!($type, $lhs, $rhs, |);
            bin_test!($type, $lhs, $rhs, ^);
        };
    }
    
    #[test]
    fn test_i8_bitops() {
        all_bit_tests!(i8, 2i8, 100i8);
        all_bit_tests!(i8, 20i8, 4i8);
        all_bit_tests!(i8, 22i8, 5i8);
    }

    #[test]
    fn test_i16_bitops() {
        all_bit_tests!(i16, 100i16, 101i16);
        all_bit_tests!(i16, 20i16, 4i16);
        all_bit_tests!(i16, 22i16, 5i16);
    }

    #[test]
    fn test_i32_bitops() {
        all_bit_tests!(i32, 120i32, 55i32);
        all_bit_tests!(i32, 20i32, 4i32);
        all_bit_tests!(i32, 22i32, 5i32);
    }

    #[test]
    fn test_i64_bitops() {
        all_bit_tests!(i64, 100i64, 50i64);
        all_bit_tests!(i64, 20i64, 4i64);
        all_bit_tests!(i64, 26i64, 100i64);
    }

    #[test]
    fn test_u8_bitops() {
        all_bit_tests!(u8, 100u8, 50u8);
        all_bit_tests!(u8, 20u8, 4u8);
        all_bit_tests!(u8, 2u8, 5u8);
    }

    #[test]
    fn test_u16_bitops() {
        all_bit_tests!(u16, 100u16, 50u16);
        all_bit_tests!(u16, 20u16, 4u16);
        all_bit_tests!(u16, 2u16, 5u16);
    }

    #[test]
    fn test_u32_bitops() {
        all_bit_tests!(u32, 100u32, 50u32);
        all_bit_tests!(u32, 20u32, 4u32);
        all_bit_tests!(u32, 2u32, 5u32);
    }

    #[test]
    fn test_u64_bitops() {
        all_bit_tests!(u64, 100u64, 50u64);
        all_bit_tests!(u64, 20u64, 4u64);
        all_bit_tests!(u64, 2u64, 5u64);
    }
    // std::ops::Shl tests

    macro_rules! shift_tests {
        ($type:ty, $lhs:expr, $rhs:expr) => {
            bin_test!($type, $lhs, $rhs, <<);
            bin_test!($type, $lhs, $rhs, >>);
        };
    }

    #[test]
    fn test_i8_shift() {
        shift_tests!(i8, 100i8, 3i8);
        shift_tests!(i8, 20i8, 4i8);
        shift_tests!(i8, 22i8, 5i8);
    }

    #[test]
    fn test_i16_shift() {
        shift_tests!(i16, 100i16, 3i16);
        shift_tests!(i16, 20i16, 4i16);
        shift_tests!(i16, 22i16, 5i16);
    }

    #[test]
    fn test_i32_shift() {
        shift_tests!(i32, 100i32, 3i32);
        shift_tests!(i32, 20i32, 4i32);
        shift_tests!(i32, 22i32, 5i32);
    }

    #[test]
    fn test_i64_shift() {
        shift_tests!(i64, 100i64, 3i64);
        shift_tests!(i64, 20i64, 4i64);
        shift_tests!(i64, 22i64, 5i64);
    }

    #[test]
    fn test_u8_shift() {
        shift_tests!(u8, 100u8, 3u8);
        shift_tests!(u8, 20u8, 4u8);
        shift_tests!(u8, 22u8, 5u8);
    }

    #[test]
    fn test_u16_shift() {
        shift_tests!(u16, 100u16, 3u16);
        shift_tests!(u16, 20u16, 4u16);
        shift_tests!(u16, 22u16, 5u16);
    }

    #[test]
    fn test_u32_shift() {
        shift_tests!(u32, 100u32, 3u32);
        shift_tests!(u32, 20u32, 4u32);
        shift_tests!(u32, 22u32, 5u32);
    }

    #[test]
    fn test_u64_shift() {
        shift_tests!(u64, 100u64, 3u64);
        shift_tests!(u64, 20u64, 4u64);
        shift_tests!(u64, 22u64, 5u64);
    }

    // std::ops::Not tests

    macro_rules! unary_test {
        ($type:ty, $val:expr, $op:tt) => {
            let value = Value::from($val);
            let expected = Value::from($op$val);
            let actual = $op value;
            assert_eq!(expected, actual);
        };
    }

    macro_rules! not_tests {
        ($type:ty, $val:expr) => {
            unary_test!($type, $val, !);
        };
    }

    #[test]
    fn test_i8_not() {
        not_tests!(i8, 100i8);
        not_tests!(i8, 20i8);
        not_tests!(i8, 22i8);
    }

    #[test]
    fn test_i16_not() {
        not_tests!(i16, 1245i16);
        not_tests!(i16, 1005i16);
    }

    #[test]
    fn test_i32_not() {
        not_tests!(i32, 10000i32);
        not_tests!(i32, 5000i32);
    }

    #[test]
    fn test_i64_not() {
        not_tests!(i64, 100000i64);
        not_tests!(i64, 50000i64);
    }

    #[test]
    fn test_u8_not() {
        not_tests!(u8, 100u8);
        not_tests!(u8, 20u8);
        not_tests!(u8, std::u8::MAX);
        not_tests!(u8, std::u8::MIN);
    }

    #[test]
    fn test_u16_not() {
        not_tests!(u16, 1000u16);
        not_tests!(u16, 2000u16);
        not_tests!(u16, std::u16::MAX);
        not_tests!(u16, std::u16::MIN);
    }

    #[test]
    fn test_u32_not() {
        not_tests!(u32, std::u32::MAX);
        not_tests!(u32, std::u32::MIN);
    }

    #[test]
    fn test_u64_not() {
        not_tests!(u64, std::u64::MAX);
        not_tests!(u64, std::u64::MIN);
    }

    #[test]
    fn test_bool_not() {
        not_tests!(bool, true);
        not_tests!(bool, false);
    }

    // std::ops::Neg tests

    macro_rules! neg_tests {
        ($type:ty, $val:expr) => {
            unary_test!($type, $val, -);
        };
    }
    

    #[test]
    fn test_i8_neg() {
        neg_tests!(i8, 100i8);
        neg_tests!(i8, -20i8);
    }

    #[test]
    fn test_i16_neg() {
        neg_tests!(i16, 1245i16);
        neg_tests!(i16, -1005i16);
    }

    #[test]
    fn test_i32_neg() {
        neg_tests!(i32, 10000i32);
        neg_tests!(i32, -5000i32);
    }

    #[test]
    fn test_i64_neg() {
        neg_tests!(i64, 100000i64);
        neg_tests!(i64, -50000i64);
    }

    #[test]
    fn test_f32_neg() {
        neg_tests!(f32, std::f32::consts::PI);
        neg_tests!(f32, -std::f32::consts::E);
    }

    #[test]
    fn test_f64_neg() {
        neg_tests!(f64, std::f64::consts::PI);
        neg_tests!(f64, -std::f64::consts::E);
    }
}
