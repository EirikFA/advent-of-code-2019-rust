use std::path::PathBuf;

use crate::intcode::program::Program;

pub fn run_day_9() {
  let program = Program::from(&PathBuf::from("src/input/day_9.txt"));
  run_day_9_1(&mut program.clone());
  run_day_9_2(&mut program.clone());
}

fn run_day_9_1(program: &mut Program) {
  let boost_keycode: isize = program
    .run(Some(vec![1]))
    .unwrap()
    .get(0)
    .copied()
    .expect("No output from BOOST program");
  println!("BOOST test mode keycode: {}", boost_keycode);
}

fn run_day_9_2(program: &mut Program) {
  let coordinates: isize = program
    .run(Some(vec![2]))
    .unwrap()
    .get(0)
    .copied()
    .expect("No output from BOOST program");
  println!("Coordinates of distress signal: {:?}", coordinates);
}
