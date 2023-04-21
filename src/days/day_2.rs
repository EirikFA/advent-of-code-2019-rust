use crate::Result;
use std::{fs, path::PathBuf};

pub fn parse_intcode_program(path: &PathBuf) -> Result<Vec<usize>> {
  let program = fs::read_to_string(path)?;
  Ok(
    program
      .trim()
      .split(',')
      .map(|s| s.parse::<usize>())
      .collect::<std::result::Result<Vec<_>, _>>()?,
  )
}

pub fn restore_intcode_program(program: &mut Vec<usize>) {
  program[1] = 12;
  program[2] = 2;
}

fn do_instruction(opcode: usize, operands: [usize; 2]) -> Result<usize> {
  match opcode {
    1 => Ok(operands[0] + operands[1]),
    2 => Ok(operands[0] * operands[1]),
    _ => Err(format!("Invalid opcode: {}", opcode).into()),
  }
}

pub fn run_intcode(program: &mut Vec<usize>) -> Result<Vec<usize>> {
  let mut pointer = 0;
  loop {
    let opcode = program[pointer];
    if opcode == 99 {
      break;
    }

    let operands = [program[program[pointer + 1]], program[program[pointer + 2]]];
    let result_param = program[pointer + 3];
    program[result_param] = do_instruction(opcode, operands)?;

    pointer += 4;
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
      let output = run_intcode(&mut program)?;
      if output[0] == TARGET_OUTPUT {
        return Ok((noun, verb));
      }
    }
  }

  Err("No noun and verb found that produce the target output".into())
}
