use crate::{
    instruction::Instruction,
};

fn fold_slice(code: &[Instruction]) -> Option<Vec<Instruction>> {
    match code {
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IAdd] => Some(vec![
            Instruction::ILoad(a + b),
        ]),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::ISub] => Some(vec![
            Instruction::ILoad(a - b),
        ]),
        _ => None,
    }
}

pub fn fold_consts(code: Vec<Instruction>) -> Vec<Instruction> {
    let mut new_code: Vec<Instruction> = vec![];
    let mut i = 0;
    let len = code.len();
    while i < len {
        if i + 3 <= len {
            if let Some(folded) = fold_slice(&code[i..i + 3]) {
                new_code.extend(folded);
                i += 3;
                continue;
            }
        }
        new_code.push(code[i].clone());
        i += 1;
    }
    // if new_code is different from code run new_code through recursively
    if new_code != code {
        fold_consts(new_code)
    }else{
        new_code
    }
}
