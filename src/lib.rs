mod instruction;
mod machine;
mod gensym;
mod constant;
mod const_table;


#[cfg(test)]
mod tests {
    use crate::{
        machine::Machine,
        instruction::Instruction,
    };

    #[test]
    fn test_compile() {
        let mut machine = Machine::new();
        let instr = vec![
            Instruction::I_LOAD(1),
            Instruction::I_LOAD(2),
            Instruction::I_ADD,
        ];
        machine.compile(instr);
    }
}
