mod const_fold;
mod const_table;
mod const_type;
mod gensym;
mod instruction;
mod machine;

#[cfg(test)]
mod tests {
    use crate::{instruction::Instruction, machine::Machine, const_fold::fold_consts};

    #[test]
    fn test_eq() {
        let mut machine = Machine::new();
        let instr = vec![
            Instruction::ILoad(1),
            Instruction::ILoad(1),
            Instruction::IEq,
            Instruction::BPrint,
        ];
        machine.load_instrs(instr.clone());
        machine.run();
        let code = machine.get_code();
        assert_eq!(
            code,
            vec![
                "int gensym0 = 1;\n",
                "int gensym1 = 1;\n",
                "bool gensym2 = gensym0 == gensym1;\n",
                "printf(\"%s\\n\", gensym2 ? \"true\" : \"false\");\n",
            ]
        );
    }

    #[test]
    fn test_fold_consts() {
        let code = vec![
            Instruction::ILoad(1),
            Instruction::ILoad(2),
            Instruction::IAdd,
            Instruction::ILoad(3),
            Instruction::IAdd,
        ];
        let folded = fold_consts(code);
        assert_eq!(folded, vec![Instruction::ILoad(6),]);
    }
}
