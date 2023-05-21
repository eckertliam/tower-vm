use crate::{
    instruction::Instruction,
    value::{
        Type,
        Value,
    },
    gensym::Gensym,
};

pub struct Machine {
    stack: Vec<Value>,
    ip: usize,
    program: Vec<Instruction>,
    code: Vec<String>,
    gensym: Gensym,
}

impl Machine {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            stack: Vec::new(),
            ip: 0,
            program,
            code: Vec::new(),
            gensym: Gensym::new(),
        }
    }

    fn push_var(&mut self, value: Value) {
        let id = self.gensym.next();
        let (code, type_) = value.as_var(&id);
        self.code.push(code);
        self.stack.push(Value::Symbol(id, type_));
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn add(&mut self) {
        let mut rhs = self.stack.pop().unwrap();
        let mut lhs = self.stack.pop().unwrap();
        if !lhs.is_symbol() {
            self.push_var(lhs);
            lhs = self.stack.pop().unwrap();
        }
        if !rhs.is_symbol() {
            self.push_var(rhs);
            rhs = self.stack.pop().unwrap();
        }
        match (lhs.get_type(), rhs.get_type()) {
            (Type::Int, Type::Int) => {
                let id = self.gensym.next();
                let code = format!("int {} = {} + {};", id, lhs, rhs);
                self.code.push(code);
                self.push(Value::Symbol(id, Type::Int));
            },
            (Type::Float, Type::Float) => {
                let id = self.gensym.next();
                let code = format!("float {} = {} + {};", id, lhs, rhs);
                self.code.push(code);
                self.push(Value::Symbol(id, Type::Float));
            },
            _ => panic!("Type mismatch {:?} + {:?}", lhs, rhs),
        }
    }

    pub fn emit(&mut self) {
        let instruction = self.program[self.ip].clone();
        match instruction {
            Instruction::Const(value) => self.push_var(value),
            Instruction::Add => self.add(),
            _ => (),
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            self.emit();
            self.ip += 1;
        }
    }

    pub fn get_code(&self) -> Vec<String> {
        self.code.clone()
    }
}
