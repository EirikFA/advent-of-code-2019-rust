use crate::{
  intcode::{instruction::Instruction, opcode::Opcode, Program},
  Result,
};
use std::{fs, path::PathBuf};

pub fn parse_intcode_program(path: &PathBuf) -> Result<Program> {
  let program = fs::read_to_string(path)?;
  Ok(
    program
      .trim()
      .split(',')
      .map(|s| s.parse::<isize>())
      .collect::<std::result::Result<Vec<_>, _>>()?,
  )
}

pub fn restore_intcode_program(program: &mut Program) {
  program[1] = 12;
  program[2] = 2;
}

pub fn run_intcode(program: &mut Program, inputs: Option<Vec<isize>>) -> Result<Program> {
  let mut pointer = 0;
  let mut input_pointer = 0;
  loop {
    let opcode = Opcode::from_first_value(program[pointer] as usize);
    if opcode == Opcode::Halt {
      break;
    }

    let instruction =
      Instruction::from_ints(&program[pointer..=pointer + opcode.parameter_count()]);
    let unwrapped_inputs = inputs.clone().unwrap_or(vec![]);
    let new_pointer: Option<usize> =
      instruction.run(program, unwrapped_inputs.get(input_pointer).copied());

    pointer = new_pointer.unwrap_or(pointer + opcode.parameter_count() + 1);
    if instruction.opcode == Opcode::SaveInput {
      input_pointer += 1;
    }
  }

  Ok(program.to_vec())
}

pub const TARGET_OUTPUT: usize = 19690720;
pub fn find_noun_verb() -> Result<(usize, usize)> {
  let original_program = parse_intcode_program(&PathBuf::from("src/input/day_2.txt"))?;
  for noun in 0..=99 {
    for verb in 0..=99 {
      let mut program = original_program.clone();
      program[1] = noun;
      program[2] = verb;
      let output = run_intcode(&mut program, None)?;
      if output[0] == TARGET_OUTPUT as isize {
        return Ok((noun as usize, verb as usize));
      }
    }
  }

  Err("No noun and verb found that produce the target output".into())
}
