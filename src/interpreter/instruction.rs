#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    // machine control instructions
    Halt,    // stop execution
    SetType, // set the type flag the value of the next byte
    GetType, // push the type flag to the stack as a byte

    // arithmetic instructions: pop two values, convert to the type flag, perform the operation, push the result
    Add,
    Sub,
    Mul,
    Div,
    Rem,

    // Neg does not pop a value, it just negates the top of the stack
    Neg,

    // incr and decr don't pop a value, they just increment or decrement the top of the stack
    // will probably throw err if called on a non-integer type
    // good for control flow, address arithmetic, etc.
    Incr,
    Decr,

    // comparison instructions: pop two values, perform the comparison, push boolean result
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,

    // bitwise instructions: pop two values, perform the operation, push the result
    And,
    Or,
    Xor,
    Shl,
    Shr,

    // bitwise not does not pop a value, it just negates the top of the stack
    Not,

    // control flow
    Jmp,      // set ip to address popped from stack
    JmpIf, // pop two values, if the first is true, set ip to the second, otherwise continue (type flag is ignored everything is treated as u64)
    JmpIfNot, // pop two values, if the first is false, set ip to the second, otherwise continue (type flag is ignored everything is treated as u64)

    // function call instructions
    Call, // pop address from stack, push return address, set ip to address
    Ret,  // pop return address from stack, set ip to address (same as jmp, but rather be explicit)

    // stack manipulation instructions
    Push, // based on the type flag, read the following bytes from the code and push them to the stack, and adjust the ip by the number of bytes read
    Dup,  // duplicate the top of the stack
    Drop, // drop the top of the stack
    Swap, // swap the top two values on the stack

    // heap instructions
    Load,  // pop address from stack and push value at address using the current alignment
    Store, // pop value and address from stack, store value at address as bytes
    Alloc, // pop size from stack, expand heap by sizeof(current type flag) * size, push first address of new heap space
    Free,  // pop address from stack, decrement heap size by sizeof(current type flag) * size
    HeapSize, // push heap size to stack
    StackSize, // push stack size to stack

    // meta instructions
    // these instructions are used for loading code into the interpreter
    LoadCode, // pop two addresses from stack, take the bytes between the addresses on the heap and extend the code segment with them, and push the address of the new code segment to the stack
    SaveCode, // pop two addresses from stack, take the bytes between the addresses on the code segment and extend the heap with them, and push the address of the new heap segment to the stack

    // io instructions
    Read, // halts execution and awaits input to the stream
    Write, // pops a value from the stack and writes it to the stream
    Print, // print the stream
    Clear, // clear the stream
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        use Instruction::*;

        match value {
            0 => Halt,
            1 => SetType,
            2 => GetType,
            3 => Add,
            4 => Sub,
            5 => Mul,
            6 => Div,
            7 => Rem,
            8 => Neg,
            9 => Incr,
            10 => Decr,
            11 => Eq,
            12 => Neq,
            13 => Lt,
            14 => Gt,
            15 => Lte,
            16 => Gte,
            17 => And,
            18 => Or,
            19 => Xor,
            20 => Shl,
            21 => Shr,
            22 => Not,
            23 => Jmp,
            24 => JmpIf,
            25 => JmpIfNot,
            26 => Call,
            27 => Ret,
            28 => Push,
            29 => Dup,
            30 => Drop,
            31 => Swap,
            32 => Load,
            33 => Store,
            34 => Alloc,
            35 => Free,
            36 => HeapSize,
            37 => StackSize,
            38 => LoadCode,
            39 => SaveCode,
            40 => Read,
            41 => Write,
            42 => Print,
            43 => Clear,
            _ => panic!("invalid instruction"),
        }
    }
}
