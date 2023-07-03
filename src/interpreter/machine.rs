use super::value::Value;

const STACK_SIZE: usize = 1024;

pub struct Machine {
    stack: [u64; STACK_SIZE],
    sp: usize,
    code: Vec<u8>,
    ip: usize,
    heap: Vec<Value>,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            sp: 0,
            code: Vec::new(),
            ip: 0,
            heap: Vec::new(),
        }
    }
}