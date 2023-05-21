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
            Instruction::I_LOAD(8),
            Instruction::I_LOAD(4),
            Instruction::I_SHL,
            Instruction::I_PRINT,
        ];
        machine.compile(instr);
    }

    #[test]
    fn test_eq() {
        let mut machine = Machine::new();
        let instr = vec![
            Instruction::I_LOAD(8),
            Instruction::I_LOAD(8),
            Instruction::I_EQ,
            Instruction::B_PRINT,
        ];
        machine.compile(instr);
    }

    #[test]
    fn test_floats() {
        let mut machine = Machine::new();
        let instr = vec![
            Instruction::F_LOAD(8.5),
            Instruction::F_LOAD(8.5),
            Instruction::F_ADD,
            Instruction::F_PRINT,
            Instruction::F_LOAD(8.5),
            Instruction::F_LOAD(8.5),
            Instruction::F_SUB,
            Instruction::F_LOAD(0.0),
            Instruction::F_EQ,
            Instruction::B_PRINT,
        ];
        machine.compile(instr);
    }

    #[test]
    fn test_if() {
        let mut machine = Machine::new();
        let instr = vec![
            Instruction::I_LOAD(8),
            Instruction::I_LOAD(8),
            Instruction::I_EQ,
            Instruction::B_IF(vec![
                Instruction::I_LOAD(1),
                Instruction::I_LOAD(1),
                Instruction::I_PRINT,
            ]),
            Instruction::I_LOAD(0),
            Instruction::I_PRINT,
        ];
        machine.compile(instr);
    }
}
