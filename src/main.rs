use crate::days::{
  day_2::{self, parse_intcode_program, restore_intcode_program, run_intcode},
  day_3, day_4, day_5,
};
use std::path::PathBuf;

mod days;
mod intcode;
mod util;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
  println!(
    "Total rocket fuel requirement: {}",
    days::day_1::total_fuel_requirement().unwrap()
  );

  let mut program = parse_intcode_program(&PathBuf::from("src/input/day_2.txt")).unwrap();
  restore_intcode_program(&mut program);
  println!(
    "Value at position 0 after running intcode: {}",
    run_intcode(&mut program, None).unwrap()[0]
  );

  let (noun, verb) = day_2::find_noun_verb().unwrap();
  println!(
    "Noun and verb that produce the target output {}: {}",
    day_2::TARGET_OUTPUT,
    100 * noun + verb
  );

  let intersection = day_3::closest_wire_intersection().unwrap();
  println!(
    "Closest intersection distance: {}",
    day_3::manhattan_distance(intersection.0, intersection.1)
  );

  let lowest_steps = day_3::lowest_steps_to_intersection().unwrap();
  println!("Lowest steps to intersection: {}", lowest_steps);

  println!(
    "Amount of possible passwords in range {} - {}: {}",
    day_4::LOWER_LIMIT,
    day_4::UPPER_LIMIT,
    day_4::possible_passwords_count()
  );

  day_5::run_day_5();
}
