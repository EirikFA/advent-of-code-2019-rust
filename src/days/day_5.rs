use std::path::PathBuf;

use crate::days::day_2::{parse_intcode_program, run_intcode};

pub fn run_day_5() {
  let mut air_conditioner_program =
    parse_intcode_program(&PathBuf::from("src/input/day_5.txt")).unwrap();
  println!("Running Thermal Environment Supervision Terminal (TEST) program for air conditioner..");
  run_intcode(&mut air_conditioner_program, Some(vec![1])).unwrap();
  println!("TEST program finished running");

  let mut thermal_radiator_program =
    parse_intcode_program(&PathBuf::from("src/input/day_5.txt")).unwrap();
  println!(
    "Running Thermal Environment Supervision Terminal (TEST) program for thermal radiator.."
  );
  run_intcode(&mut thermal_radiator_program, Some(vec![5])).unwrap();
  println!("TEST program finished running");
}
