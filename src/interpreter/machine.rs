use super::value::{TypeFlag, Value};

const STACK_SIZE: usize = 1024;


macro_rules! binop_helper {
    ($fn_name:ident, $op:tt) => {
        fn $fn_name(&mut self) {
            let lhs = self.value_pop();
            let rhs = self.value_pop();
            self.value_push(lhs $op rhs);
        }
    };
}

macro_rules! comp_helper {
    ($fn_name:ident, $op:tt) => {
        fn $fn_name(&mut self) {
            let lhs = self.value_pop();
            let rhs = self.value_pop();
            let curr_ty = self.ty_flag;
            self.ty_flag = TypeFlag::Bool;
            self.value_push((lhs $op rhs).into());
            self.ty_flag = curr_ty;
        }
    };
}

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

    binop_helper!(add, +);

    binop_helper!(sub, -);

    binop_helper!(mul, *);

    binop_helper!(div, /);

    binop_helper!(rem, %);

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

    comp_helper!(eq, ==);
    comp_helper!(neq, !=);
    comp_helper!(lt, <);
    comp_helper!(gt, >);
    comp_helper!(lte, <=);
    comp_helper!(gte, >=);

    binop_helper!(and, &);
    binop_helper!(or, |);
    binop_helper!(xor, ^);
    binop_helper!(shl, <<);
    binop_helper!(shr, >>);

    fn not(&mut self) {
        self.stack[self.sp - 1] = (!Value::from_stack(self.ty_flag, self.stack[self.sp - 1])).to_stack();
    }

    fn jmp(&mut self) {
        let addr = self.stack_pop() as usize;
        self.ip = addr;
    }

    fn jmp_if(&mut self) {
        let cond = self.value_pop();
        let addr = self.stack_pop() as usize;

        if cond.to_stack() != 0 {
            self.ip = addr;
        }
    }

    fn jmp_if_not(&mut self) {
        let cond = self.value_pop();
        let addr = self.stack_pop() as usize;

        if cond.to_stack() == 0 {
            self.ip = addr;
        }
    }

    fn call(&mut self) {
        let addr = self.stack_pop() as usize;
        self.stack_push(self.ip as u64);
        self.ip = addr;
    }

    fn ret(&mut self) {
        self.ip = self.stack_pop() as usize;
    }

    fn push(&mut self) {
        self.ip += 1;
        let value = Value::from_code(self.ty_flag, &self.code[self.ip..self.ip + self.ty_flag.size()]);
        self.value_push(value);
        self.ip += self.ty_flag.size() - 1;
    }

    fn dup(&mut self) {
        self.stack[self.sp] = self.stack[self.sp - 1];
        self.sp += 1;
    }

    fn drop(&mut self) {
        self.sp -= 1;
    }

    fn swap(&mut self) {
        let tmp = self.stack[self.sp - 1];
        self.stack[self.sp - 1] = self.stack[self.sp - 2];
        self.stack[self.sp - 2] = tmp;
    }

    fn load(&mut self) {
        let addr = self.stack_pop() as usize;
        let value = Value::from_code(self.ty_flag, &self.heap[addr..addr + self.ty_flag.size()]);
        self.value_push(value);
    }

    fn store(&mut self) {
        let addr = self.stack_pop() as usize;
        let value = self.value_pop();
        let raw = value.to_code();
        for i in 0..self.ty_flag.size() {
            self.heap[addr + i] = raw[i];
        }
    }

    fn alloc(&mut self) {
        let size = self.stack_pop() as usize;
        let addr = self.heap.len();
        self.heap.resize(addr + size, 0);
        self.stack_push(addr as u64);
    }

    fn free(&mut self) {
        let addr = self.stack_pop() as usize;
        let size = self.stack_pop() as usize;
        self.heap.resize(addr + size, 0);
    }

}
