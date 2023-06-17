use std::path::PathBuf;

use crate::intcode::program::Program;

pub fn run_day_5() {
  run_air_conditioner();
  run_thermal_radiator();
}

fn run_air_conditioner() {
  let mut air_conditioner_program = Program::from(&PathBuf::from("src/input/day_5.txt"));
  let output = air_conditioner_program.run(Some(vec![1])).unwrap();
  println!(
    "TEST program for air conditioner finished running with output: {:?}",
    output
  );
}

fn run_thermal_radiator() {
  let mut thermal_radiator_program = Program::from(&PathBuf::from("src/input/day_5.txt"));
  let output = thermal_radiator_program.run(Some(vec![5])).unwrap();
  println!(
    "TEST program for thermal radiator finished running with output: {:?}",
    output
  );
}
