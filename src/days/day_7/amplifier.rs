use crate::intcode::{instruction::InstructionResult, opcode::Opcode, program::Program};

pub struct Amplifier {
  program: Program,
  phase: Option<u32>,
}

impl Amplifier {
  pub fn new(program: Program, phase: u32) -> Self {
    Self {
      program,
      phase: Some(phase),
    }
  }

  pub fn run(&mut self, signal: u32) -> Option<u32> {
    loop {
      let opcode = Opcode::from_first_value(self.program.get_current() as usize);
      if opcode == Opcode::Halt {
        break;
      }

      let instruction = self.program.get_instruction(opcode.parameter_count());
      let result: InstructionResult = self.program.run_instruction(
        &instruction,
        Some(self.phase.map_or(signal as isize, |p| p as isize)),
      );

      if opcode == Opcode::SaveInput && self.phase.is_some() {
        self.phase = None;
      }

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
