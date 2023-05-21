use crate::intcode::{
  instruction::{Instruction, InstructionResult},
  opcode::Opcode,
  Program,
};

pub struct Amplifier {
  program: Program,
  pointer: usize,
  phase: Option<u32>,
}

impl Amplifier {
  pub fn new(program: Program, phase: u32) -> Self {
    Self {
      program,
      pointer: 0,
      phase: Some(phase),
    }
  }

  pub fn run(&mut self, signal: u32) -> Option<u32> {
    loop {
      let opcode = Opcode::from_first_value(self.program[self.pointer] as usize);
      if opcode == Opcode::Halt {
        break;
      }

      let instruction = Instruction::from_ints(
        &self.program[self.pointer..=self.pointer + opcode.parameter_count()],
      );

      let result: InstructionResult = instruction.run(
        &mut self.program,
        Some(self.phase.map_or(signal as isize, |p| p as isize)),
      );

      if opcode == Opcode::SaveInput && self.phase.is_some() {
        self.phase = None;
      }

      self.pointer = result
        .pointer
        .unwrap_or(self.pointer + opcode.parameter_count() + 1);

      if let Some(output) = result.output {
        return Some(
          output
            .try_into()
            .expect("Amplifier did not return a u32, check program."),
        );
      }
    }

    None
  }
}
