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

    fn stack_push(&mut self, raw: u64) {
        // push a raw value to the stack and increment the stack pointer
        self.stack[self.sp] = raw;
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> u64 {
        // get the raw value from the stack and decrement the stack pointer
        self.sp -= 1;
        self.stack[self.sp]
    }

    fn value_pop(&mut self) -> Value {
        // pop a value from the stack and convert it to the current type flag
        Value::from_stack(self.ty_flag, self.stack_pop())
    }

    fn value_push(&mut self, value: Value) {
        // push a value to the stack 
        self.stack_push(value.to_stack());
    }

    fn halt(self) {
        std::process::exit(0);
    }

    fn set_ty_flag(&mut self) {
        // ip currently points to the SET_TYPE instruction
        // increment ip to point to the next byte which is the type flag
        self.ip += 1;
        self.ty_flag = self.code[self.ip].into();
        self.ip += 1;
    }

    fn get_ty_flag(&mut self) {
        self.stack_push(self.ty_flag.into());            
    }

    fn add(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        self.value_push(lhs + rhs);
    }

    fn sub(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        self.value_push(lhs - rhs);
    }

    fn mul(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        self.value_push(lhs * rhs);
    }

    fn div(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        self.value_push(lhs / rhs);
    }

    fn rem(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        self.value_push(lhs % rhs);
    }

    fn neg(&mut self) {
        self.stack[self.sp - 1] = (-(Value::from_stack(self.ty_flag, self.stack[self.sp - 1]))).to_stack();
    }

    fn incr(&mut self) {
        let value = Value::from_stack(self.ty_flag, self.stack[self.sp - 1]);
        self.stack[self.sp - 1] = (value + Value::from_stack(self.ty_flag, 1)).to_stack();
    }

    fn decr(&mut self) {
        let value = Value::from_stack(self.ty_flag, self.stack[self.sp - 1]);
        self.stack[self.sp - 1] = (value - Value::from_stack(self.ty_flag, 1)).to_stack();
    }

    fn eq(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        let curr_ty = self.ty_flag;
        self.ty_flag = TypeFlag::Bool;
        self.value_push((lhs == rhs).into());
        self.ty_flag = curr_ty;
    }

    fn neq(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        let curr_ty = self.ty_flag;
        self.ty_flag = TypeFlag::Bool;
        self.value_push((lhs != rhs).into());
        self.ty_flag = curr_ty;
    }

    fn lt(&mut self) {
        let lhs = self.value_pop();
        let rhs = self.value_pop();
        let curr_ty = self.ty_flag;
        self.ty_flag = TypeFlag::Bool;
        self.value_push((lhs < rhs).into());
        self.ty_flag = curr_ty;
    }
}
