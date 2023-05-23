use crate::{
    instruction::Instruction,
};

fn fold_slice(code: &[Instruction]) -> Option<Instruction> {
    match code {
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IAdd] => Some(Instruction::ILoad(a + b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::ISub] => Some(Instruction::ILoad(a - b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IMul] => Some(Instruction::ILoad(a * b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IDiv] => Some(Instruction::ILoad(a / b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IMod] => Some(Instruction::ILoad(a % b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IEq] => Some(Instruction::BLoad(a == b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::INeq] => Some(Instruction::BLoad(a != b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::ILt] => Some(Instruction::BLoad(a < b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::ILte] => Some(Instruction::BLoad(a <= b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IGt] => Some(Instruction::BLoad(a > b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IGte] => Some(Instruction::BLoad(a >= b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IAnd] => Some(Instruction::ILoad(a & b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IOr] => Some(Instruction::ILoad(a | b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IXor] => Some(Instruction::ILoad(a ^ b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IShl] => Some(Instruction::ILoad(a << b)),
        &[Instruction::ILoad(a), Instruction::ILoad(b), Instruction::IShr] => Some(Instruction::ILoad(a >> b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FAdd] => Some(Instruction::FLoad(a + b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FSub] => Some(Instruction::FLoad(a - b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FMul] => Some(Instruction::FLoad(a * b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FDiv] => Some(Instruction::FLoad(a / b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FMod] => Some(Instruction::FLoad(a % b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FEq] => Some(Instruction::BLoad(a == b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FNeq] => Some(Instruction::BLoad(a != b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FLt] => Some(Instruction::BLoad(a < b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FLte] => Some(Instruction::BLoad(a <= b)),
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FGt] => Some(Instruction::BLoad(a > b)), 
        &[Instruction::FLoad(a), Instruction::FLoad(b), Instruction::FGte] => Some(Instruction::BLoad(a >= b)),
        &[Instruction::BLoad(a), Instruction::BLoad(b), Instruction::BAnd] => Some(Instruction::BLoad(a && b)),
        &[Instruction::BLoad(a), Instruction::BLoad(b), Instruction::BOr] => Some(Instruction::BLoad(a || b)),
        &[Instruction::BLoad(a), Instruction::BLoad(b), Instruction::BEq] => Some(Instruction::BLoad(a == b)),
        &[Instruction::BLoad(a), Instruction::BLoad(b), Instruction::BNeq] => Some(Instruction::BLoad(a != b)),
        &[Instruction::BLoad(a), Instruction::BNot] => Some(Instruction::BLoad(!a)),
        _ => None,
    }
}

pub fn fold_consts(code: Vec<Instruction>) -> Vec<Instruction> {
    let mut new_code: Vec<Instruction> = vec![];
    let mut i = 0;
    let len = code.len();
    while i < len {
        if i + 2 <= len {
            if let Some(folded) = fold_slice(&code[i..i + 2]) {
                new_code.push(folded);
                i += 2;
                continue;
            }
        }
        if i + 3 <= len {
            if let Some(folded) = fold_slice(&code[i..i + 3]) {
                new_code.push(folded);
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
