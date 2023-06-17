use crate::intcode::program::Program;

use super::amplifier::Amplifier;
use itertools::Itertools;
use std::path::PathBuf;

pub fn run_day_7_1() {
  let amplifier_program = Program::from(&PathBuf::from("src/input/day_7.txt"));
  let signal = max_thruster_signal(amplifier_program);
  println!("Highest possible thruster signal: {:?}", signal);
}

fn run_amplifier(phase: u32, signal: u32, program: &mut Program) -> u32 {
  let mut amp = Amplifier::new(program.clone(), phase);
  let signal = amp.run(signal);
  signal.expect("Missing output signal from amplifier, check program")
}

fn thruster_signal(phases: Vec<u32>, program: &mut Program) -> u32 {
  let mut signal = 0;
  for phase in phases {
    signal = run_amplifier(phase, signal, program);
  }
  signal
}

fn max_thruster_signal(program: Program) -> u32 {
  let mut highest = 0;
  for phases in (0..=4).permutations(5) {
    let signal = thruster_signal(phases, &mut (program.clone()));
    if signal > highest {
      highest = signal;
    }
  }

  highest
}
