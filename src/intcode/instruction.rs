use std::io::{self, Write};

use super::{
  opcode::Opcode,
  parameter::{Parameter, ParameterMode},
  Program,
};

pub struct Instruction {
  pub opcode: Opcode,
  parameters: Vec<Parameter>,
}

impl Instruction {
  pub fn from_ints(ints: &[isize]) -> Instruction {
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

  fn map_parameter_values(&self, program: Program) -> Vec<isize> {
    self
      .parameters
      .iter()
      .map(|p| p.get_value(&program))
      .collect::<Vec<_>>()
  }

  fn set_result(&self, program: &mut Program, result: isize) {
    let result_param = self.parameters.last().unwrap();
    program[result_param.address_or_value as usize] = result;
  }

  /// Optionally returns a new instruction pointer
  pub fn run(&self, program: &mut Program, input: Option<isize>) -> Option<usize> {
    match self.opcode {
      Opcode::Add => {
        let operands: Vec<isize> = self.map_parameter_values(program.to_vec());
        self.set_result(program, operands[0] + operands[1]);
        None
      }
      Opcode::Multiply => {
        let operands: Vec<isize> = self.map_parameter_values(program.to_vec());
        self.set_result(program, operands[0] * operands[1]);
        None
      }
      Opcode::SaveInput => {
        if let Some(input) = input {
          self.set_result(program, input);
          return None;
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
        None
      }
      Opcode::Output => {
        let parameter = &self.parameters[0];
        println!("{}", parameter.get_value(program));
        None
      }
      Opcode::JumpIfTrue => {
        let params: Vec<isize> = self.map_parameter_values(program.to_vec());
        if params[0] != 0 {
          return Some(
            params[1]
              .try_into()
              .expect("Invalid jump address (must be positive)"),
          );
        }
        None
      }
      Opcode::JumpIfFalse => {
        let params: Vec<isize> = self.map_parameter_values(program.to_vec());
        if params[0] == 0 {
          return Some(
            params[1]
              .try_into()
              .expect("Invalid jump address (must be positive)"),
          );
        }
        None
      }
      Opcode::LessThan => {
        let params: Vec<isize> = self.map_parameter_values(program.to_vec());
        self.set_result(program, (params[0] < params[1]) as isize);
        None
      }
      Opcode::Equals => {
        let params: Vec<isize> = self.map_parameter_values(program.to_vec());
        self.set_result(program, (params[0] == params[1]) as isize);
        None
      }
      Opcode::Halt => None,
    }
  }
}
