use super::{
    instruction::Instruction,
    value::{TypeFlag, Value},
};

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


macro_rules! bytecode {
    ( $( $item:expr ),* ) => {
        {
            let mut tmp_vec: Vec<u8> = vec![];

            $(
                tmp_vec.push($item as u8);
            )*

            tmp_vec
        }
    }
}

pub struct Machine {
    stack: [u64; STACK_SIZE],
    sp: usize,
    code: Vec<u8>,
    ip: usize,
    heap: Vec<u8>,
    ty_flag: TypeFlag, // 0 = i8, 1 = i16, 2 = i32, 3 = i64, 4 = f32, 5 = f64, 6 = bool, 7 = char, 8 = u8, 9 = u16, 10 = u32, 11 = u64
}

impl Machine {
    pub fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            sp: 0,
            code: Vec::new(),
            ip: 0,
            heap: Vec::new(),
            ty_flag: 11.into(), // default to u64
        }
    }

    fn zero(&mut self) {
        self.stack.fill(0);
        self.sp = 0;
        self.code = vec![];
        self.ip = 0;
        self.heap = vec![];
        self.ty_flag = 11.into();
    }

    fn push_code(&mut self, code: &[u8]) {
        self.code.extend_from_slice(code)
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

    fn halt(&self) {
        std::process::exit(0);
    }

    fn set_ty_flag(&mut self) {
        // ip currently points to the SET_TYPE instruction
        // increment ip to point to the next byte which is the type flag
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
        self.stack[self.sp - 1] =
            (-(Value::from_stack(self.ty_flag, self.stack[self.sp - 1]))).to_stack();
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
        self.stack[self.sp - 1] =
            (!Value::from_stack(self.ty_flag, self.stack[self.sp - 1])).to_stack();
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
        let value = Value::from_code(
            self.ty_flag,
            &self.code[self.ip..self.ip + self.ty_flag.size()],
        );
        self.value_push(value);
        self.ip += self.ty_flag.size();
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

    fn heap_size(&mut self) {
        self.stack_push(self.heap.len() as u64);
    }

    fn stack_size(&mut self) {
        self.stack_push(self.sp as u64);
    }

    fn load_code(&mut self) {
        let addr = self.stack_pop() as usize;
        let size = self.stack_pop() as usize;
        let new_code_start = self.code.len();
        self.code.extend_from_slice(&self.heap[addr..addr + size]);
        self.stack_push(new_code_start as u64);
    }

    fn save_code(&mut self) {
        let addr = self.stack_pop() as usize;
        let size = self.stack_pop() as usize;
        let code_seg = &self.code[addr..addr + size];
        let heap_len = self.heap.len();
        self.heap.extend_from_slice(code_seg);
        self.stack_push(heap_len as u64);
    }

    fn println(&mut self) {
        let value = self.value_pop();
        println!("{}", value);
    }
    
    fn fetch_instr(&mut self) -> Instruction {
        if self.ip >= self.code.len() {
            panic!("Error: Instruction pointer has gone out of bounds")
        }else{
            self.ip += 1;
            return self.code[self.ip - 1].into()
        }
    }

    fn dispatch(&mut self) {
        use Instruction::*;

        let instr = self.fetch_instr();

        match instr {
            Halt => self.halt(),
            SetType => self.set_ty_flag(),
            GetType => self.get_ty_flag(),
            Add => self.add(),
            Sub => self.sub(),
            Mul => self.mul(),
            Div => self.div(),
            Rem => self.rem(),
            Neg => self.neg(),
            Incr => self.incr(),
            Decr => self.decr(),
            Eq => self.eq(),
            Neq => self.neq(),
            Lt => self.lt(),
            Gt => self.gt(),
            Lte => self.lte(),
            Gte => self.gte(),
            And => self.and(),
            Or => self.or(),
            Xor => self.xor(),
            Shl => self.shl(),
            Shr => self.shr(),
            Not => self.not(),
            Jmp => self.jmp(),
            JmpIf => self.jmp_if(),
            JmpIfNot => self.jmp_if_not(),
            Call => self.call(),
            Ret => self.ret(),
            Push => self.push(),//
            Dup => self.dup(),
            Drop => self.drop(),
            Swap => self.swap(),
            Load => self.load(),
            Store => self.store(),
            Alloc => self.alloc(),
            Free => self.free(),
            HeapSize => self.heap_size(),
            StackSize => self.stack_size(),
            LoadCode => self.load_code(),
            SaveCode => self.save_code(),
            Print => self.println(),
        }
    }

    fn run(&mut self) {
        while self.ip <= self.code.len() {
            self.dispatch()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_stack_ops() {
        let mut machine = Machine::new();

        machine.stack_push(10);
        assert_eq!(machine.sp, 1);

        let pop = machine.stack_pop();
        assert_eq!(pop, 10u64);
        assert_eq!(machine.sp, 0);
    }

    #[test]
    fn test_push_instr() {
        use Instruction::*;
        use TypeFlag::*;
        let mut machine = Machine::new();

        let value: Value = 11i8.into();
        
        machine.code = bytecode!(SetType, I8, Push);
        machine.code.extend_from_slice(&value.to_code());
        machine.push_code(&bytecode!(Halt));
        machine.run();
        assert_eq!(machine.stack[machine.sp - 1], 11);
    }

    #[test]
    fn test_arith_instrs() {
        use Instruction::*;
        use TypeFlag::*;

        let mut machine = Machine::new();

        let lhs: Value = 1000i64.into();
        let rhs: Value = 500i64.into();
        let mut start = vec![];
        start = bytecode!(SetType, I64, Push);
        start.extend_from_slice(&lhs.to_code());
        start.extend_from_slice(&bytecode![Push]);
        start.extend_from_slice(&rhs.to_code());
        
        let mut add_test = start.clone();
        add_test.extend_from_slice(&bytecode!(Add, Halt));
        machine.code = add_test;
        machine.run();
        assert_eq!(Value::from(1500i64), machine.value_pop());
        machine.zero();

        let mut sub_test = start.clone();
        sub_test.extend_from_slice(&bytecode!(Sub, Halt));
        machine.code = sub_test;
        machine.run();
        assert_eq!(Value::from(500i64), machine.value_pop());
        machine.zero();

        let mut mul_test = start.clone();
        mul_test.extend_from_slice(&bytecode!(Mul, Halt));
        machine.code = mul_test;
        machine.run();
        assert_eq!(Value::from(50000i64), machine.value_pop());
        machine.zero();

        let mut div_test = start.clone();
        div_test.extend_from_slice(&bytecode!(Div, Halt));
        machine.code = div_test;
        machine.run();
        assert_eq!(Value::from(2i64), machine.value_pop());
        machine.zero();

        let mut rem_test = start;
        rem_test.extend_from_slice(&bytecode!(Rem, Halt));
        machine.code = rem_test;
        machine.run();
        assert_eq!(Value::from(0i64), machine.value_pop());
    }

}
