use std::{fs, path::PathBuf};

use crate::Result;

use super::{
  instruction::{Instruction, InstructionResult},
  opcode::Opcode,
};

const MEMORY_SIZE: usize = 2000;

#[derive(Clone, Debug)]
pub struct Program {
  pub memory: Vec<isize>,
  pub pointer: usize,
  pub relative_base: isize,
}

impl From<&PathBuf> for Program {
  fn from(path: &PathBuf) -> Program {
    let initial = fs::read_to_string(path).unwrap();
    // Allocate minimum memory size
    let mut memory: Vec<isize> = Vec::with_capacity(MEMORY_SIZE);

    memory.extend(
      initial
        .trim()
        .split(',')
        .map(|s| s.parse::<isize>().unwrap()),
    );

    // Expand memory to minimum size if needed
    if memory.len() < MEMORY_SIZE {
      memory.resize(MEMORY_SIZE, 0);
    }

    Program {
      memory,
      pointer: 0,
      relative_base: 0,
    }
  }
}

impl Program {
  pub fn get(&self, address: usize) -> isize {
    self.memory[address]
  }

  pub fn set(&mut self, address: usize, value: isize) {
    self.memory[address] = value;
  }

  pub fn get_current(&self) -> isize {
    self.get(self.pointer)
  }

  pub fn get_instruction(&self, param_count: usize) -> Instruction {
    Instruction::from(&self.memory[self.pointer..=self.pointer + param_count])
  }

  pub fn run_instruction(
    &mut self,
    instruction: &Instruction,
    input: Option<isize>,
  ) -> InstructionResult {
    let result: InstructionResult = instruction.run(self, input);
    self.pointer = result
      .pointer
      .unwrap_or(self.pointer + instruction.opcode.parameter_count() + 1);
    result
  }

  pub fn adjust_relative_base(&mut self, value: isize) {
    self.relative_base += value;
  }

  pub fn run(&mut self, inputs: Option<Vec<isize>>) -> Result<Vec<isize>> {
    let mut input_pointer = 0;
    let mut outputs: Vec<isize> = vec![];
    loop {
      let opcode = Opcode::from_first_value(self.get_current() as usize);
      if opcode == Opcode::Halt {
        break;
      }

      let instruction = self.get_instruction(opcode.parameter_count());
      let unwrapped_inputs = inputs.clone().unwrap_or(vec![]);

      let result: InstructionResult =
        self.run_instruction(&instruction, unwrapped_inputs.get(input_pointer).copied());

      if let Some(output) = result.output {
        outputs.push(output);
      }

      if instruction.opcode == Opcode::SaveInput {
        input_pointer += 1;
      }
    }

    Ok(outputs)
  }
}
