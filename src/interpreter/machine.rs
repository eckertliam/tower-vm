use super::value::{TypeFlag, Value};

const STACK_SIZE: usize = 1024;


pub struct Machine {
    stack: [u64; STACK_SIZE],
    sp: usize,
    code: Vec<u8>,
    ip: usize,
    heap: Vec<u8>,
    ty_flag: TypeFlag,// 0 = i8, 1 = i16, 2 = i32, 3 = i64, 4 = f32, 5 = f64, 6 = bool, 7 = char, 8 = u8, 9 = u16, 10 = u32, 11 = u64
}

impl Machine {
    pub fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            sp: 0,
            code: Vec::new(),
            ip: 0,
            heap: Vec::new(),
            ty_flag: 11.into(),// default to u64
        }
    }
}
