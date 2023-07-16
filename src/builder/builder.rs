use std::collections::HashMap;

use super::chunk::Chunk;

use crate::{Instruction, TypeFlag, Value};

macro_rules! simple_instr {
    ($fn_name:ident, $instr:ident) => {
        pub fn $fn_name(&mut self) {
            self.push_instr(Instruction::$instr);
        }
    };
}



pub struct Builder {
    chunks: Vec<Chunk>,
    funcs: HashMap<String, u64>,// store the start point of a function with its identifier
}   

impl Builder {
    pub fn new() -> Self {
        Self {
            chunks: vec![],
            funcs: HashMap::new(),
        }
    }

    fn get_func(&self, ident: &str) -> Option<u64> {
        self.funcs.get(ident).copied()
    }

    fn push_instr(&mut self, instr: Instruction) {
        self.chunks.push(Chunk::Instr(instr))
    }

    pub fn push(&mut self, value: Value) {
        self.push_instr(Instruction::Push);
        self.chunks.push(Chunk::Literal(value));
    }

    // a utility to handle the pushing of a collection putting the first element at the top of the stack
    pub fn push_collect(&mut self, values: &mut Vec<Value>) {
        while values.len() > 0 {
            self.push(values.pop().unwrap());
        }        
    }

    // set the start point of a new function to the next index in code
    // if the function is already defined, return Err with the index of the existing function
    pub fn start_fn(&mut self, id: &str) -> Result<(), u64> {
        let ident = id.to_string();

        match self.get_func(id) {
            Some(_) => Err(self.funcs[&ident]),
            None => {
                self.funcs.insert(ident, self.chunks.len() as u64);
                Ok(())
            },
        }
    }

    // mark the stop of a function by pushing the return instruction
    simple_instr!(ret_fn, Ret);

    // mark the end of the program
    simple_instr!(halt, Halt);

    // set the type alignment
    pub fn set_type(&mut self, ty_flag: TypeFlag) {
        self.push_instr(Instruction::SetType);
        self.chunks.push(Chunk::Type(ty_flag));
    }

    simple_instr!(get_type, GetType);

    simple_instr!(add, Add);

    simple_instr!(sub, Sub);

    simple_instr!(div, Div);

    simple_instr!(rem, Rem);

    simple_instr!(neg, Neg);

    simple_instr!(incr, Incr);

    simple_instr!(decr, Decr);

    simple_instr!(eq, Eq);

    simple_instr!(neq, Neq);

    simple_instr!(lt, Lt);

    simple_instr!(gt, Gt);

    simple_instr!(lte, Lte);

    simple_instr!(gte, Gte);

    simple_instr!(and, And);

    simple_instr!(or, Or);

    simple_instr!(xor, Xor);

    simple_instr!(shl, Shl);

    simple_instr!(shr, Shr);

    simple_instr!(not, Not);

    // optionally give the address to be jumped to
    pub fn jmp(&mut self, addr: Option<u64>) {
        if let Some(idx) = addr {
            self.push(idx.into());
        }
        self.push_instr(Instruction::Jmp);
    }

    simple_instr!(jmp_if, JmpIf);

    simple_instr!(jmp_if_not, JmpIfNot);

    pub fn call(&mut self, ident: &str) -> Result<(), String> {
        match self.get_func(ident) {
            Some(idx) => {
                self.push(idx.into());
                Ok(self.push_instr(Instruction::Call))
            }
            None => Err(format!("Error: fn {} is undefined", ident)),
        }
    }

    simple_instr!(ret, Ret);

    simple_instr!(dup, Dup);

    simple_instr!(drop, Drop);

    simple_instr!(swap, Swap);

    // optionally provide the ptr to the heap address
    pub fn load(&mut self, ptr: Option<u64>) {
        if let Some(idx) = ptr {
            self.push(idx.into());
        }
        self.push_instr(Instruction::Load);
    }

    simple_instr!(store, Store);

    simple_instr!(alloc, Alloc);

    simple_instr!(free, Free);

    simple_instr!(heap_size, HeapSize);

    simple_instr!(stack_size, StackSize);

    simple_instr!(load_code, LoadCode);

    simple_instr!(save_code, SaveCode);

    simple_instr!(print, Print);

    fn to_code(&mut self) -> Vec<u8> {
        let mut code = vec![];
        
        for chunk in self.chunks.iter_mut() {
            code.append(&mut chunk.to_code());
        }

        code
    }

    // there will be optional optimization passes here in the distant future
    pub fn build(&mut self) -> Vec<u8> {
        self.to_code()
    }
}