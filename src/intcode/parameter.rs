use crate::util::{digit_count, digits};

use super::Program;

#[derive(PartialEq)]
pub enum ParameterMode {
  Position,
  Immediate,
}

impl From<usize> for ParameterMode {
  fn from(value: usize) -> ParameterMode {
    match value {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      _ => panic!("Invalid parameter mode: {}", value),
    }
  }
}

impl ParameterMode {
  pub fn from_first_value(value: usize, index: usize) -> ParameterMode {
    // Backwards compatibility for day 2 intcode implementation
    if digit_count(value as isize) <= 2 {
      return ParameterMode::Position;
    }

    let first_value_digits: Vec<usize> = digits(value);
    let param_mode_index: isize = first_value_digits.len() as isize - 3 - index as isize;

    if param_mode_index < 0 {
      return ParameterMode::Position;
    }
    ParameterMode::from(first_value_digits[param_mode_index as usize])
  }
}

pub struct Parameter {
  pub mode: ParameterMode,
  pub address_or_value: isize,
}

impl Parameter {
  pub fn get_value(&self, program: &Program) -> isize {
    match self.mode {
      // Position parameters are always addresses (and thus positive)
      ParameterMode::Position => program[self.address_or_value as usize],
      ParameterMode::Immediate => self.address_or_value,
    }
  }
}
