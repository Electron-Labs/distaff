#[cfg(test)]
use crate::{ ProofOptions, opcodes::f128 as opcodes, F128, Accumulator };

#[test]
fn execute_verify() {
    let program = [
        opcodes::SWAP, opcodes::DUP2, opcodes::DROP, opcodes::ADD,
        opcodes::SWAP, opcodes::DUP2, opcodes::DROP, opcodes::ADD,
        opcodes::SWAP, opcodes::DUP2, opcodes::DROP, opcodes::ADD,
        opcodes::NOOP, opcodes::NOOP, opcodes::NOOP, opcodes::NOOP,
    ];
    let expected_hash = <F128 as Accumulator>::digest(&program[..(program.len() - 1)]);

    let options = ProofOptions::default();
    let inputs = [1, 0];
    let num_outputs = 1;

    let (outputs, program_hash, proof) = super::execute(&program, &inputs, num_outputs, &options);
    assert_eq!(outputs, [3]);
    assert_eq!(program_hash, expected_hash);

    let result = super::verify(&program_hash, &inputs, &outputs, &proof);
    assert_eq!(Ok(true), result);
}

#[test]
fn execute_verify_fail() {
    let program = [
        opcodes::SWAP, opcodes::DUP2, opcodes::DROP, opcodes::ADD,
        opcodes::SWAP, opcodes::DUP2, opcodes::DROP, opcodes::ADD,
        opcodes::SWAP, opcodes::DUP2, opcodes::DROP, opcodes::ADD,
        opcodes::NOOP, opcodes::NOOP, opcodes::NOOP, opcodes::NOOP,
    ];
    let expected_hash = <F128 as Accumulator>::digest(&program[..(program.len() - 1)]);

    let options = ProofOptions::default();
    let inputs = [1, 0];
    let num_outputs = 1;

    let (outputs, program_hash, proof) = super::execute(&program, &inputs, num_outputs, &options);
    assert_eq!(outputs, [3]);
    assert_eq!(program_hash, expected_hash);

    // wrong inputs
    let result = super::verify(&program_hash, &[1, 1], &outputs, &proof);
    let err_msg = format!("verification of low-degree proof failed: evaluations did not match column value at depth 0");
    assert_eq!(Err(err_msg), result);

    // wrong outputs
    let result = super::verify(&program_hash, &inputs, &[5], &proof);
    let err_msg = format!("verification of low-degree proof failed: evaluations did not match column value at depth 0");
    assert_eq!(Err(err_msg), result);

    // wrong program hash
    let mut program_hash2 = program_hash.clone();
    program_hash2[0] = 1;
    let result = super::verify(&program_hash2, &inputs, &outputs, &proof);
    let err_msg = format!("verification of low-degree proof failed: evaluations did not match column value at depth 0");
    assert_eq!(Err(err_msg), result);
}

#[test]
fn stack_operations() {
    let program = [
        opcodes::SWAP, opcodes::SWAP2, opcodes::SWAP4, opcodes::CHOOSE,
        opcodes::ROLL4, opcodes::DUP, opcodes::PUSH, 11, opcodes::ROLL8,
        opcodes::DROP, opcodes::SWAP2, opcodes::CHOOSE2, opcodes::DUP2,
        opcodes::DUP4, opcodes::DROP, opcodes::NOOP
    ];
    let expected_hash = <F128 as Accumulator>::digest(&program[..(program.len() - 1)]);

    let options = ProofOptions::default();
    let inputs = [7, 6, 5, 4, 3, 2, 1, 0];
    let num_outputs = 8;

    let (outputs, program_hash, proof) = super::execute(&program, &inputs, num_outputs, &options);
    assert_eq!(outputs, [4, 11, 4, 11, 4, 11, 4, 6]);
    assert_eq!(program_hash, expected_hash);

    let result = super::verify(&program_hash, &inputs, &outputs, &proof);
    assert_eq!(Ok(true), result);
}

#[test]
fn math_operations() {
    let program = [
        opcodes::ADD, opcodes::MUL, opcodes::SWAP, opcodes::SUB,
        opcodes::ADD, opcodes::MUL, opcodes::NOOP, opcodes::NOOP,
        opcodes::NOOP, opcodes::NOOP, opcodes::NOOP, opcodes::NOOP,
        opcodes::NOOP, opcodes::NOOP, opcodes::NOOP, opcodes::NOOP,
    ];
    let expected_hash = <F128 as Accumulator>::digest(&program[..(program.len() - 1)]);

    let options = ProofOptions::default();
    let inputs = [7, 6, 5, 4, 0, 1];
    let num_outputs = 1;

    let (outputs, program_hash, proof) = super::execute(&program, &inputs, num_outputs, &options);
    assert_eq!(outputs, [61]);
    assert_eq!(program_hash, expected_hash);

    let result = super::verify(&program_hash, &inputs, &outputs, &proof);
    assert_eq!(Ok(true), result);
}

// TODO: add more tests