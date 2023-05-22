use std::vec;

use crate::{
    instruction::Instruction,
};


// takes a slice of 3 instruction and if constant folding is possible, returns the folded instruction
// otherwise returns None
fn fold_const_slice(code: &[Instruction]) -> Option<Vec<Instruction>> {
    match code {
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_ADD] => Some(vec![Instruction::I_LOAD(a + b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_SUB] => Some(vec![Instruction::I_LOAD(a - b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_MUL] => Some(vec![Instruction::I_LOAD(a * b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_DIV] => Some(vec![Instruction::I_LOAD(a / b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_MOD] => Some(vec![Instruction::I_LOAD(a % b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_SHL] => Some(vec![Instruction::I_LOAD(a << b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_SHR] => Some(vec![Instruction::I_LOAD(a >> b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_AND] => Some(vec![Instruction::I_LOAD(a & b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_OR] => Some(vec![Instruction::I_LOAD(a | b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_XOR] => Some(vec![Instruction::I_LOAD(a ^ b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_EQ] => Some(vec![Instruction::B_LOAD(a == b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_NE] => Some(vec![Instruction::B_LOAD(a != b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_LT] => Some(vec![Instruction::B_LOAD(a < b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_LE] => Some(vec![Instruction::B_LOAD(a <= b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_GT] => Some(vec![Instruction::B_LOAD(a > b)]),
        &[Instruction::I_LOAD(a), Instruction::I_LOAD(b), Instruction::I_GE] => Some(vec![Instruction::B_LOAD(a >= b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_ADD] => Some(vec![Instruction::F_LOAD(a + b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_SUB] => Some(vec![Instruction::F_LOAD(a - b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_MUL] => Some(vec![Instruction::F_LOAD(a * b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_DIV] => Some(vec![Instruction::F_LOAD(a / b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_MOD] => Some(vec![Instruction::F_LOAD(a % b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_EQ] => Some(vec![Instruction::B_LOAD(a == b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_NE] => Some(vec![Instruction::B_LOAD(a != b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_LT] => Some(vec![Instruction::B_LOAD(a < b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_LE] => Some(vec![Instruction::B_LOAD(a <= b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_GT] => Some(vec![Instruction::B_LOAD(a > b)]),
        &[Instruction::F_LOAD(a), Instruction::F_LOAD(b), Instruction::F_GE] => Some(vec![Instruction::B_LOAD(a >= b)]),
        &[Instruction::B_LOAD(a), Instruction::B_LOAD(b), Instruction::B_AND] => Some(vec![Instruction::B_LOAD(a && b)]),
        &[Instruction::B_LOAD(a), Instruction::B_LOAD(b), Instruction::B_OR] => Some(vec![Instruction::B_LOAD(a || b)]),
        &[Instruction::B_LOAD(a), Instruction::B_LOAD(b), Instruction::B_EQ] => Some(vec![Instruction::B_LOAD(a == b)]),
        &[Instruction::B_LOAD(a), Instruction::B_LOAD(b), Instruction::B_NE] => Some(vec![Instruction::B_LOAD(a != b)]),
        _ => None,
    }
}

// loops through the code and tries to fold constant expressions
fn fold_constants(code: Vec<Instruction>) -> Vec<Instruction> {
    let mut new_code = Vec::new();
    let mut i = 0;
    while i < code.len() {
        if i + 2 < code.len() {
            if let Some(folded) = fold_const_slice(&code[i..i + 3]) {
                new_code.extend(folded);
                i += 3;
            }
        }else{
            new_code.push(code[i].clone());
        }
    }
    // if code is changed, try to fold again
    if new_code.len() != code.len() {
        return fold_constants(new_code);
    }
    new_code
}

#[test]
fn test_fold() {
    let code = vec![
        Instruction::I_LOAD(1),
        Instruction::I_LOAD(2),
        Instruction::I_ADD,
        Instruction::I_LOAD(3),
        Instruction::I_ADD,
    ];
    let folded = fold_constants(code);
    assert_eq!(folded, vec![
        Instruction::I_LOAD(3),
        Instruction::I_LOAD(3),
        Instruction::I_ADD,
    ]);
}