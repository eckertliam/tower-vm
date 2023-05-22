use crate::{
    const_table::ConstantTable, 
    instruction::Instruction,
    const_type::ConstType,
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
        let fpath = "/tmp/temp.c";
        let mut file = std::fs::File::create(fpath).unwrap();
        file.write_all(code.as_bytes()).unwrap();
        // compile temporary file
        let output = Command::new("cc")
            .arg(fpath)
            .arg("-o")
            .arg("/tmp/temp")
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            panic!("failed to compile");
        }
        // execute temporary file
        let output = Command::new("/tmp/temp")
            .stdout(Stdio::piped())
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            panic!("failed to execute");
        }
        // print output
        io::stdout().write_all(&output.stdout).unwrap();
    }
    
    pub fn load_instrs(&mut self, instrs: Vec<Instruction>) {
        self.instrs = instrs;
    }

    pub fn assemble(&mut self) -> String {
        self.run();
        let body = self.code.join("");
        let header = "#include <stdio.h>\n
                      #include <stdbool.h>\n
                      #include <stdint.h>\n
                      #include <inttypes.h>\n";
        format!("{} int main() {{\n{}\n}}", header, body)
    }

    fn push(&mut self, symbol: &str) {
        self.stack.push(symbol.to_string());
    }

    fn pop(&mut self) -> Option<String> {
        self.stack.pop()
    }

    // pop a symbol from the stack expecting a type
    // if the symbol is not of the expected type, return None
    fn expect(&mut self, type_: ConstType) -> Result<String, String> {
        let symbol = self.pop().ok_or("Stack underflow")?;
        let const_t = self.constants.get(&symbol).unwrap();
        if const_t == &type_ {
            Ok(symbol)
        } else {
            Err(format!("Expected type {:?} but found {:?}", type_, const_t))
        }
    }

    fn push_constant(&mut self, type_: ConstType) -> String {
        self.constants.push(type_)
    }

    fn load_int(&mut self, i: i32) {
        let symbol = self.push_constant(ConstType::I32);
        self.push(&symbol);
        self.code.push(format!("int {} = {};\n", symbol, i));
    }

    fn load_float(&mut self, f: f32) {
        let symbol = self.push_constant(ConstType::F32);
        self.push(&symbol);
        self.code.push(format!("float {} = {};\n", symbol, f));
    }

    fn load_bool(&mut self, b: bool) {
        let symbol = self.push_constant(ConstType::Bool);
        self.push(&symbol);
        self.code.push(format!("bool {} = {};\n", symbol, b));
    }

    fn binary_op(&mut self, op: &str, lhs_t: ConstType, rhs_t: ConstType, result_type: ConstType) {
        let right = self.expect(rhs_t);
        let left = self.expect(lhs_t);
        match (left, right) {
            (Ok(left), Ok(right)) => {
                let symbol = self.push_constant(result_type);
                let t_str = result_type.to_string();
                self.code.push(format!("{} {} = {} {} {};\n", t_str, symbol, left, op, right));
                self.push(&symbol);
            }
            (Err(e), _) | (_, Err(e)) => panic!("{}", e),
        }
    }

    fn int_add(&mut self) {
        self.binary_op("+", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_sub(&mut self) {
        self.binary_op("-", ConstType::I32, ConstType::I32, ConstType::I32)
    }

    fn int_mul(&mut self) {
        self.binary_op("*", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_div(&mut self) {
        self.binary_op("/", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_mod(&mut self) {
        self.binary_op("%", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_shl(&mut self) {
        self.binary_op("<<", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_shr(&mut self) {
        self.binary_op(">>", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_and(&mut self) {
        self.binary_op("&", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_or(&mut self) {
        self.binary_op("|", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_xor(&mut self) {
        self.binary_op("^", ConstType::I32, ConstType::I32, ConstType::I32);
    }

    fn int_eq(&mut self) {
        self.binary_op("==", ConstType::I32, ConstType::I32, ConstType::Bool);
    }

    fn int_neq(&mut self) {
        self.binary_op("!=", ConstType::I32, ConstType::I32, ConstType::Bool);
    }

    fn int_lt(&mut self) {
        self.binary_op("<", ConstType::I32, ConstType::I32, ConstType::Bool);
    }

    fn int_lte(&mut self) {
        self.binary_op("<=", ConstType::I32, ConstType::I32, ConstType::Bool);
    }

    fn int_gt(&mut self) {
        self.binary_op(">", ConstType::I32, ConstType::I32, ConstType::Bool);
    }

    fn int_gte(&mut self) {
        self.binary_op(">=", ConstType::I32, ConstType::I32, ConstType::Bool);
    }


    fn int_print(&mut self) {
        let symbol = self.expect(ConstType::I32).unwrap();
        self.code.push(format!("printf(\"%d\\n\", {});\n", symbol));
    }

    fn bool_print(&mut self) {
        let symbol = self.expect(ConstType::Bool).unwrap();
        self.code.push(format!("printf(\"%s\\n\", {} ? \"true\" : \"false\");\n", symbol));
    }

    fn bool_not(&mut self) {
        let val = self.expect(ConstType::Bool).unwrap();
        let symbol = self.push_constant(ConstType::Bool);
        self.code.push(format!("bool {} = !{};\n", symbol, val));
        self.push(&symbol);
    }

    fn bool_and(&mut self) {
        self.binary_op("&&", ConstType::Bool, ConstType::Bool, ConstType::Bool);
    }

    fn bool_or(&mut self) {
        self.binary_op("||", ConstType::Bool, ConstType::Bool, ConstType::Bool);
    }

    fn bool_eq(&mut self) {
        self.binary_op("==", ConstType::Bool, ConstType::Bool, ConstType::Bool);
    }

    fn bool_neq(&mut self) {
        self.binary_op("!=", ConstType::Bool, ConstType::Bool, ConstType::Bool);
    }

    fn float_add(&mut self) {
        self.binary_op("+", ConstType::F32, ConstType::F32, ConstType::F32);
    }

    fn float_sub(&mut self) {
        self.binary_op("-", ConstType::F32, ConstType::F32, ConstType::F32);
    }

    fn float_mul(&mut self) {
        self.binary_op("*", ConstType::F32, ConstType::F32, ConstType::F32);
    }

    fn float_div(&mut self) {
        self.binary_op("/", ConstType::F32, ConstType::F32, ConstType::F32);
    }

    fn float_mod(&mut self) {
        self.binary_op("%", ConstType::F32, ConstType::F32, ConstType::F32);
    }

    fn float_eq(&mut self) {
        self.binary_op("==", ConstType::F32, ConstType::F32, ConstType::Bool);
    }

    fn float_neq(&mut self) {
        self.binary_op("!=", ConstType::F32, ConstType::F32, ConstType::Bool);
    }

    fn float_lt(&mut self) {
        self.binary_op("<", ConstType::F32, ConstType::F32, ConstType::Bool);
    }

    fn float_gt(&mut self) {
        self.binary_op(">", ConstType::F32, ConstType::F32, ConstType::Bool);
    }

    fn float_leq(&mut self) {
        self.binary_op("<=", ConstType::F32, ConstType::F32, ConstType::Bool);
    }

    fn float_geq(&mut self) {
        self.binary_op(">=", ConstType::F32, ConstType::F32, ConstType::Bool);
    }

    fn float_print(&mut self) {
        let value = self.expect(ConstType::F32);
        match value {
            Ok(value) => {
                self.code.push(format!("printf(\"%f\\n\", {});\n", value));
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn bool_if(&mut self, block: Vec<Instruction>) {
        let cond = self.expect(ConstType::Bool);
        match cond {
            Ok(cond) => {
                self.code.push(format!("if ({}) {{\n", cond));
                // determine the length of the stack before the block is evaluated
                let mut len = self.stack.len();
                // evaluate the block
                for instr in block {
                    self.eval(&instr);
                }
                // pop the stack until it is the same length as before the block was evaluated
                while self.stack.len() > len {
                    self.pop();
                }
                self.code.push("}\n".to_string());
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn eval(&mut self, instr: &Instruction) {
        match instr {
            Instruction::I_LOAD(i) => self.load_int(*i),
            Instruction::F_LOAD(f) => self.load_float(*f),
            Instruction::B_LOAD(b) => self.load_bool(*b),
            Instruction::I_ADD => self.int_add(),
            Instruction::I_SUB => self.int_sub(),
            Instruction::I_MUL => self.int_mul(),
            Instruction::I_DIV => self.int_div(),
            Instruction::I_MOD => self.int_mod(),
            Instruction::I_SHL => self.int_shl(),
            Instruction::I_SHR => self.int_shr(),
            Instruction::I_AND => self.int_and(),
            Instruction::I_OR => self.int_or(),
            Instruction::I_XOR => self.int_xor(),
            Instruction::I_EQ => self.int_eq(),
            Instruction::I_NE => self.int_neq(),
            Instruction::I_LT => self.int_lt(),
            Instruction::I_LE => self.int_lte(),
            Instruction::I_GT => self.int_gt(),
            Instruction::I_GE => self.int_gte(),
            Instruction::I_PRINT => self.int_print(),
            Instruction::B_PRINT => self.bool_print(),
            Instruction::B_NOT => self.bool_not(),
            Instruction::B_AND => self.bool_and(),
            Instruction::B_OR => self.bool_or(),
            Instruction::B_EQ => self.bool_eq(),
            Instruction::B_NE => self.bool_neq(),
            Instruction::F_ADD => self.float_add(),
            Instruction::F_SUB => self.float_sub(),
            Instruction::F_MUL => self.float_mul(),
            Instruction::F_DIV => self.float_div(),
            Instruction::F_MOD => self.float_mod(),
            Instruction::F_EQ => self.float_eq(),
            Instruction::F_NE => self.float_neq(),
            Instruction::F_LT => self.float_lt(),
            Instruction::F_LE => self.float_leq(),
            Instruction::F_GT => self.float_gt(),
            Instruction::F_GE => self.float_geq(),
            Instruction::F_PRINT => self.float_print(),
            Instruction::B_IF(block) => self.bool_if(block.clone()),
            _ => panic!("unimplemented {:?}", instr),
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.instrs.len() {
            let instr = self.instrs[self.ip].clone();
            self.eval(&instr);
            self.ip += 1;
        }
    }
    
}