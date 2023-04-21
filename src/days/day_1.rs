use crate::util::read_lines;
use crate::Result;
use std::path::PathBuf;

fn parse_module_masses(path: &PathBuf) -> Result<Vec<u32>> {
  let lines = read_lines(path)?;
  let mut masses: Vec<u32> = Vec::new();
  for line in lines {
    masses.push(line?.parse::<u32>()?);
  }

  Ok(masses)
}

fn module_fuel_requirement(mass: u32) -> u32 {
  if mass / 3 <= 2 {
    return 0;
  }

  let fuel = (mass / 3) - 2;
  fuel + module_fuel_requirement(fuel)
}

pub fn total_fuel_requirement() -> Result<u32> {
  let masses: Vec<u32> = parse_module_masses(&PathBuf::from("src/input/day_1.txt"))?;
  Ok(
    masses
      .iter()
      .map(|mass| module_fuel_requirement(*mass))
      .sum::<u32>(),
  )
}
