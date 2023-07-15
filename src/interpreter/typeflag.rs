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
    pub fn to_code(self) -> Vec<u8> {
        // first byte is instruction TYPE, second byte is the type flag
        vec![1, self.into()]
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
            _ => panic!("bad typeflag: {}", byte),
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
