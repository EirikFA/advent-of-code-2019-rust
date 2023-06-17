use std::io::{self, Write};

use super::{
  opcode::Opcode,
  parameter::{Parameter, ParameterMode},
  program::Program,
};

pub struct Instruction {
  pub opcode: Opcode,
  parameters: Vec<Parameter>,
}

pub struct InstructionResult {
  pub output: Option<isize>,
  pub pointer: Option<usize>,
}

impl InstructionResult {
  fn empty() -> InstructionResult {
    InstructionResult {
      output: None,
      pointer: None,
    }
  }

  fn from_output(output: isize) -> InstructionResult {
    InstructionResult {
      output: Some(output),
      pointer: None,
    }
  }

  fn from_jump(pointer: usize) -> InstructionResult {
    InstructionResult {
      output: None,
      pointer: Some(pointer),
    }
  }
}

impl From<&[isize]> for Instruction {
  fn from(ints: &[isize]) -> Instruction {
    // First value must be positive
    let opcode = Opcode::from_first_value(ints[0] as usize);

    let parameters: Vec<Parameter> = (0..=opcode.parameter_count() - 1)
      .map(|i| Parameter {
        mode: ParameterMode::from_first_value(ints[0] as usize, i),
        // Skip first value/opcode
        address_or_value: ints[i + 1],
      })
      .collect::<Vec<_>>();

    Instruction { opcode, parameters }
  }
}

impl Instruction {
  fn map_parameter_values(&self, program: &Program) -> Vec<isize> {
    self
      .parameters
      .iter()
      .map(|p| p.get_value(&program))
      .collect::<Vec<_>>()
  }

  fn set_result(&self, program: &mut Program, result: isize) {
    let result_param = self.parameters.last().unwrap();
    program.set(result_param.address_or_value as usize, result);
  }

  pub fn run(&self, program: &mut Program, input: Option<isize>) -> InstructionResult {
    match self.opcode {
      Opcode::Add => {
        let operands: Vec<isize> = self.map_parameter_values(program);
        self.set_result(program, operands[0] + operands[1]);
        InstructionResult::empty()
      }
      Opcode::Multiply => {
        let operands: Vec<isize> = self.map_parameter_values(program);
        self.set_result(program, operands[0] * operands[1]);
        InstructionResult::empty()
      }
      Opcode::SaveInput => {
        if let Some(input) = input {
          self.set_result(program, input);
          return InstructionResult::empty();
        }

        let mut input_buffer = String::new();
        print!("Enter input: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let input = input_buffer
          .trim()
          .parse::<isize>()
          .expect("Invalid input, must be a number");

        self.set_result(program, input);
        InstructionResult::empty()
      }
      Opcode::Output => {
        let parameter = &self.parameters[0];
        InstructionResult::from_output(parameter.get_value(program))
      }
      Opcode::JumpIfTrue => {
        let params: Vec<isize> = self.map_parameter_values(program);
        if params[0] != 0 {
          let pointer: usize = params[1]
            .try_into()
            .expect("Invalid jump address (must be positive)");
          return InstructionResult::from_jump(pointer);
        }
        InstructionResult::empty()
      }
      Opcode::JumpIfFalse => {
        let params: Vec<isize> = self.map_parameter_values(program);
        if params[0] == 0 {
          let pointer: usize = params[1]
            .try_into()
            .expect("Invalid jump address (must be positive)");
          return InstructionResult::from_jump(pointer);
        }
        InstructionResult::empty()
      }
      Opcode::LessThan => {
        let params: Vec<isize> = self.map_parameter_values(program);
        self.set_result(program, (params[0] < params[1]) as isize);
        InstructionResult::empty()
      }
      Opcode::Equals => {
        let params: Vec<isize> = self.map_parameter_values(program);
        self.set_result(program, (params[0] == params[1]) as isize);
        InstructionResult::empty()
      }
      Opcode::Halt => InstructionResult::empty(),
    }
  }
}
