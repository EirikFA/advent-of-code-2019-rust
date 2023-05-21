use crate::days::day_2::parse_intcode_program;
use crate::days::day_7::amplifier::Amplifier;
use crate::intcode::Program;
use itertools::Itertools;
use std::path::PathBuf;

pub fn run_day_7_2() {
  let amplifier_program = parse_intcode_program(&PathBuf::from("src/input/day_7.txt")).unwrap();
  let signal = max_thruster_signal(amplifier_program);
  println!(
    "Highest possible thruster signal with feedback loop: {}",
    signal
  );
}

fn max_thruster_signal(program: Program) -> u32 {
  let mut highest = 0;
  for phases in (5..=9).permutations(5) {
    let amplifiers: Vec<Amplifier> = phases
      .into_iter()
      .map(|phase| Amplifier::new(program.clone(), phase))
      .collect();

    let signal = run_feedback_loop(amplifiers);
    if signal > highest {
      highest = signal;
    }
  }

  highest
}

fn run_feedback_loop(mut amplifiers: Vec<Amplifier>) -> u32 {
  let mut signal = 0;

  loop {
    for amp in amplifiers.iter_mut() {
      let result = amp.run(signal);

      if let Some(result) = result {
        signal = result;
      } else {
        return signal;
      }
    }
  }
}
