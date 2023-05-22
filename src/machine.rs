use crate::{
    const_table::ConstantTable, 
    instruction::Instruction,
    const_type::ConstType,
    const_fold::fold_consts,
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

    pub fn get_code(&self) -> Vec<String> {
        self.code.clone()
    }

    pub fn compile(&mut self, instrs: Vec<Instruction>, run: bool, emit: bool, keep_src: bool) {
        self.load_instrs(instrs);
        self.run();
        let code = self.assemble();
        // create temporary file
        let mut fpath = "/tmp/temp.c";
        if keep_src {
            fpath = "./exec.c";
        }
        let mut file = std::fs::File::create(fpath).unwrap();
        file.write_all(code.as_bytes()).unwrap();
        let mut exec_name = "/tmp/temp".to_string();
        if emit {
            exec_name = "./exec".to_string();
        }
        // compile temporary file
        let output = Command::new("cc")
            .arg(fpath)
            .arg("-o")
            .arg(&exec_name)
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            panic!("failed to compile");
        }
        if run {
            // execute temporary file
            let output = Command::new(exec_name)
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
    }
    
    pub fn load_instrs(&mut self, instrs: Vec<Instruction>) {
        self.instrs = fold_consts(instrs);
    }

    pub fn assemble(&mut self) -> String {
        self.run();
        let body = self.code.join("");
        let header = "#include <stdio.h>\n#include <stdbool.h>\n#include <stdint.h>\n#include <inttypes.h>\n";
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
            Instruction::ILoad(i) => self.load_int(*i),
            Instruction::FLoad(f) => self.load_float(*f),
            Instruction::BLoad(b) => self.load_bool(*b),
            Instruction::IAdd => self.int_add(),
            Instruction::ISub => self.int_sub(),
            Instruction::IMul => self.int_mul(),
            Instruction::IDiv => self.int_div(),
            Instruction::IMod => self.int_mod(),
            Instruction::IShl => self.int_shl(),
            Instruction::IShr => self.int_shr(),
            Instruction::IAnd => self.int_and(),
            Instruction::IOr => self.int_or(),
            Instruction::IXor => self.int_xor(),
            Instruction::IEq => self.int_eq(),
            Instruction::INeq => self.int_neq(),
            Instruction::ILt => self.int_lt(),
            Instruction::ILte => self.int_lte(),
            Instruction::IGt => self.int_gt(),
            Instruction::IGte => self.int_gte(),
            Instruction::IPrint => self.int_print(),
            Instruction::BPrint => self.bool_print(),
            Instruction::BNot => self.bool_not(),
            Instruction::BAnd => self.bool_and(),
            Instruction::BOr => self.bool_or(),
            Instruction::BEq => self.bool_eq(),
            Instruction::BNeq => self.bool_neq(),
            Instruction::FAdd => self.float_add(),
            Instruction::FSub => self.float_sub(),
            Instruction::FMul => self.float_mul(),
            Instruction::FDiv => self.float_div(),
            Instruction::FMod => self.float_mod(),
            Instruction::FEq => self.float_eq(),
            Instruction::FNeq => self.float_neq(),
            Instruction::FLt => self.float_lt(),
            Instruction::FLte => self.float_leq(),
            Instruction::FGt => self.float_gt(),
            Instruction::FGte => self.float_geq(),
            Instruction::FPrint => self.float_print(),
            Instruction::BIf(block) => self.bool_if(block.clone()),
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