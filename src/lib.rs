mod interpreter;
mod builder;


pub use interpreter::{
    Instruction,
    Machine,
    TypeFlag,
    Value,
};

pub use builder::Builder;