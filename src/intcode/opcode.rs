use crate::util::{digit_count, digits};

#[derive(PartialEq)]
pub enum Opcode {
  Add,
  Multiply,
  SaveInput,
  Output,
  JumpIfTrue,
  JumpIfFalse,
  LessThan,
  Equals,
  Halt,
}

impl From<usize> for Opcode {
  fn from(value: usize) -> Opcode {
    match value {
      1 => Opcode::Add,
      2 => Opcode::Multiply,
      3 => Opcode::SaveInput,
      4 => Opcode::Output,
      5 => Opcode::JumpIfTrue,
      6 => Opcode::JumpIfFalse,
      7 => Opcode::LessThan,
      8 => Opcode::Equals,
      99 => Opcode::Halt,
      _ => panic!("Invalid opcode: {}", value),
    }
  }
}

impl Opcode {
  pub fn parameter_count(&self) -> usize {
    match self {
      Opcode::Add => 3,
      Opcode::Multiply => 3,
      Opcode::SaveInput => 1,
      Opcode::Output => 1,
      Opcode::JumpIfTrue => 2,
      Opcode::JumpIfFalse => 2,
      Opcode::LessThan => 3,
      Opcode::Equals => 3,
      Opcode::Halt => 0,
    }
  }

  pub fn from_first_value(value: usize) -> Opcode {
    // Backwards compatibility for day 2 intcode implementation
    if digit_count(value as isize) <= 2 {
      return Opcode::from(value);
    }

    let first_value_digits: Vec<usize> = digits(value);
    let opcode_num =
      first_value_digits.last().unwrap() + 10 * first_value_digits[first_value_digits.len() - 2];
    Opcode::from(opcode_num)
  }
}
