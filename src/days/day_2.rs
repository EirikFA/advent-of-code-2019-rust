use crate::{
  util::{digit_count, digits},
  Result,
};
use std::{fs, path::PathBuf};

type Program = Vec<isize>;

#[derive(PartialEq)]
enum Opcode {
  Add,
  Multiply,
  SaveInput,
  Output,
  Halt,
}

impl From<usize> for Opcode {
  fn from(value: usize) -> Opcode {
    match value {
      1 => Opcode::Add,
      2 => Opcode::Multiply,
      3 => Opcode::SaveInput,
      4 => Opcode::Output,
      99 => Opcode::Halt,
      _ => panic!("Invalid opcode: {}", value),
    }
  }
}

impl Opcode {
  fn parameter_count(&self) -> usize {
    match self {
      Opcode::Add => 3,
      Opcode::Multiply => 3,
      Opcode::SaveInput => 1,
      Opcode::Output => 1,
      Opcode::Halt => 0,
    }
  }

  fn from_first_value(value: usize) -> Opcode {
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

#[derive(PartialEq)]
enum ParameterMode {
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
  fn from_first_value(value: usize, index: usize) -> ParameterMode {
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

struct Parameter {
  mode: ParameterMode,
  address_or_value: isize,
}

impl Parameter {
  fn get_value(&self, program: &Program) -> isize {
    match self.mode {
      // Position parameters are always addresses (and thus positive)
      ParameterMode::Position => program[self.address_or_value as usize],
      ParameterMode::Immediate => self.address_or_value,
    }
  }
}

struct Instruction {
  opcode: Opcode,
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

  fn get_operands(&self, program: Program) -> Vec<isize> {
    self
      .parameters
      .iter()
      .take(self.parameters.len() - 1)
      .map(|p| p.get_value(&program))
      .collect::<Vec<_>>()
  }

  fn set_result(&self, program: &mut Program, result: isize) {
    let result_param = &self.parameters[self.parameters.len() - 1];
    program[result_param.address_or_value as usize] = result;
  }

  fn run(&self, program: &mut Program) {
    match self.opcode {
      Opcode::Add => {
        let operands: Vec<isize> = self.get_operands(program.to_vec());
        self.set_result(program, operands[0] + operands[1]);
      }
      Opcode::Multiply => {
        let operands: Vec<isize> = self.get_operands(program.to_vec());
        self.set_result(program, operands[0] * operands[1]);
      }
      Opcode::SaveInput => {
        let input = 1;
        self.set_result(program, input);
      }
      Opcode::Output => {
        let parameter = &self.parameters[0];
        println!("{}", parameter.get_value(program));
      }
      Opcode::Halt => (),
    }
  }
}

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

pub fn run_intcode(program: &mut Program) -> Result<Program> {
  let mut pointer = 0;
  loop {
    let opcode = Opcode::from_first_value(program[pointer] as usize);
    if opcode == Opcode::Halt {
      break;
    }

    let instruction =
      Instruction::from_ints(&program[pointer..=pointer + opcode.parameter_count()]);
    instruction.run(program);

    pointer += opcode.parameter_count() + 1;
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
      if output[0] == TARGET_OUTPUT as isize {
        return Ok((noun as usize, verb as usize));
      }
    }
  }

  Err("No noun and verb found that produce the target output".into())
}
