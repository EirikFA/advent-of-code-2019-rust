use crate::{intcode::program::Program, Result};
use std::path::PathBuf;

pub fn restore_intcode_program(program: &mut Program) {
  program.memory[1] = 12;
  program.memory[2] = 2;
}

pub const TARGET_OUTPUT: usize = 19690720;
pub fn find_noun_verb() -> Result<(usize, usize)> {
  let original_program = Program::from(&PathBuf::from("src/input/day_2.txt"));
  for noun in 0..=99 {
    for verb in 0..=99 {
      let mut program = original_program.clone();
      program.memory[1] = noun;
      program.memory[2] = verb;
      program.run(None)?;
      if program.memory[0] == TARGET_OUTPUT as isize {
        return Ok((noun as usize, verb as usize));
      }
    }
  }

  Err("No noun and verb found that produce the target output".into())
}
