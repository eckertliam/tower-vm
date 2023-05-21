use crate::{
    const_table::ConstantTable, 
    instruction::Instruction,
    constant::{Constant, Type},
};

use std::process::{Command, Stdio};
use std::io::{self, Write};

pub struct Machine {
    stack: Vec<String>,
    constants: ConstantTable,
    code: Vec<String>,
    instrs: Vec<Instruction>,
    ip: usize,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            constants: ConstantTable::new(),
            code: Vec::new(),
            instrs: Vec::new(),
            ip: 0,
        }
    }

    pub fn compile(&mut self, instrs: Vec<Instruction>) {
        self.load_instrs(instrs);
        self.run();
        let code = self.assemble();
        // create temporary file
        let fpath = "temp.c";
        let mut file = std::fs::File::create(fpath).unwrap();
        file.write_all(code.as_bytes()).unwrap();
        // compile temporary file
        let output = Command::new("cc")
            .arg(fpath)
            .arg("-o")
            .arg("temp")
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            panic!("failed to compile");
        }
        std::fs::remove_file(fpath).unwrap();
    }
    
    pub fn load_instrs(&mut self, instrs: Vec<Instruction>) {
        self.instrs = instrs;
    }

    pub fn assemble(&mut self) -> String {
        self.run();
        let body = self.code.join("");
        format!("int main() {{\n{}\n}}", body)
    }

    fn push(&mut self, symbol: &str) {
        self.stack.push(symbol.to_string());
    }

    fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }

    // pop a symbol from the stack expecting a type
    // if the symbol is not of the expected type, return None
    fn expect(&mut self, type_: Type) -> Result<String, String> {
        let symbol = self.pop().ok_or("Stack underflow")?;
        let constant = self.constants.get(&symbol).unwrap();
        if constant.get_type() == type_ {
            Ok(symbol)
        } else {
            Err(format!("Expected type {:?} but found {:?}", type_, constant.get_type()))
        }
    }

    fn push_constant(&mut self, constant: Constant) -> String {
        self.constants.push(constant)
    }

    fn load_int(&mut self, i: i32) {
        let symbol = self.push_constant(Constant::I32(i));
        self.push(&symbol);
        self.code.push(format!("int {} = {};\n", symbol, i));
    }

    fn load_float(&mut self, f: f32) {
        let symbol = self.push_constant(Constant::F32(f));
        self.push(&symbol);
        self.code.push(format!("float {} = {};\n", symbol, f));
    }

    fn load_bool(&mut self, b: bool) {
        let symbol = self.push_constant(Constant::Bool(b));
        self.push(&symbol);
        self.code.push(format!("bool {} = {};\n", symbol, b));
    }

    fn int_add(&mut self) {
        let right = self.expect(Type::I32);
        let left = self.expect(Type::I32);
        match (left, right) {
            (Ok(left), Ok(right)) => {
                let symbol = self.push_constant(Constant::I32(0));
                self.code.push(format!("int {} = {} + {};\n", symbol, left, right));
                self.push(&symbol);
            }
            (Err(e), _) | (_, Err(e)) => panic!("{}", e),
        }
    }

    fn eval(&mut self) {
        let instr = &self.instrs[self.ip];
        match instr {
            Instruction::I_LOAD(i) => self.load_int(*i),
            Instruction::F_LOAD(f) => self.load_float(*f),
            Instruction::B_LOAD(b) => self.load_bool(*b),
            Instruction::I_ADD => self.int_add(),
            _ => panic!("unimplemented"),
        }
        self.ip += 1;
    }

    pub fn run(&mut self) {
        while self.ip < self.instrs.len() {
            self.eval();
        }
    }
    
}