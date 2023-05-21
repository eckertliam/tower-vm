mod instruction;
mod value;
mod machine;
mod gensym;

#[cfg(test)]
mod tests {

    use crate::{
        instruction::Instruction,
        value::Value,
        machine::Machine,
    };
    #[test]
    fn test_add() {
        let program = vec![
            Instruction::Const(Value::Int(1)),
            Instruction::Const(Value::Int(2)),
            Instruction::Add,
        ];
        let mut machine = Machine::new(program);
        machine.run();
        assert_eq!(machine.get_code(), vec![
            "int gensym0 = 1;",
            "int gensym1 = 2;",
            "int gensym2 = gensym0 + gensym1;",
        ]);
    }
}