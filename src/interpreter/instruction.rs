#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // machine control instructions
    Halt, // stop execution
    Type, // set the type flag the value of the next byte

    // arithmetic instructions: pop two values, convert to the type flag, perform the operation, push the result
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Pow,
    
    // incr and decr don't pop a value, they just increment or decrement the top of the stack
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
    Jmp, // set ip to address popped from stack
    JmpIf, // pop two values, if the first is true, set ip to the second, otherwise continue

    // function call instructions
    Call, // pop address from stack, push return address, set ip to address
    Ret, // pop return address from stack, set ip to address (same as jmp, but rather be explicit)
    
    // stack manipulation instructions
    Push, // based on the type flag, read the following bytes from the code and push them to the stack, and adjust the ip by the number of bytes read
    Dup, // duplicate the top of the stack
    Drop, // drop the top of the stack
    Swap, // swap the top two values on the stack

    // heap instructions
    Load, // pop address from stack
    Store, // pop value and address from stack, store value at address as bytes
    Alloc, // pop size from stack, expand heap by sizeof(current type flag) * size, push first address of new heap space
    Free, // pop address from stack, decrement heap size by sizeof(current type flag) * size

    // io instructions
    Print, // pop value from stack, print it to stdout as specified by the type flag
    Read, // read a line from stdin, push it to the stack as chars
}