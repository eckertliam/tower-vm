mod const_table;
mod const_type;
mod gensym;
mod instruction;
mod machine;

#[cfg(test)]
mod tests {
    use crate::{
        instruction::Instruction, 
        machine::Machine,
    };

    #[test]
    fn test_eq() {
        let mut machine = Machine::new();
        let instr = vec![
            Instruction::I_LOAD(1),
            Instruction::I_LOAD(1),
            Instruction::I_EQ,
            Instruction::B_PRINT,
        ];
        machine.load_instrs(instr.clone());
        machine.run();
        let code = machine.get_code();
        assert_eq!(code, vec![
            "int gensym0 = 1;\n",
            "int gensym1 = 1;\n",
            "bool gensym2 = gensym0 == gensym1;\n",
            "printf(\"%s\\n\", gensym2 ? \"true\" : \"false\");\n",
        ]);
    }
}
