use std::path::PathBuf;

use crate::days::day_2::{parse_intcode_program, run_intcode};

pub fn run_day_5() {
  run_air_conditioner();
  run_thermal_radiator();
}

fn run_air_conditioner() {
  let mut air_conditioner_program =
    parse_intcode_program(&PathBuf::from("src/input/day_5.txt")).unwrap();
  let (_, output) = run_intcode(&mut air_conditioner_program, Some(vec![1])).unwrap();
  println!(
    "TEST program for air conditioner finished running with output: {:?}",
    output
  );
}

fn run_thermal_radiator() {
  let mut thermal_radiator_program =
    parse_intcode_program(&PathBuf::from("src/input/day_5.txt")).unwrap();
  let (_, output) = run_intcode(&mut thermal_radiator_program, Some(vec![5])).unwrap();
  println!(
    "TEST program for thermal radiator finished running with output: {:?}",
    output
  );
}
