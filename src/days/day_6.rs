use crate::{util::read_lines, Result};
use std::{collections::HashMap, path::PathBuf};

type OrbitMap = HashMap<String, Option<String>>;

pub fn run_day_6() {
  let map = parse_orbit_map(&PathBuf::from("src/input/day_6.txt")).unwrap();
  let checksum: u32 = orbit_count_checksum(&map);
  println!("Day 6, part 1 - orbit map checksum: {}", checksum);

  let minimum_transfers = min_orbital_transfers("YOU", "SAN", &map);
  println!(
    "Day 6, part 2 - minimum orbital transfers: {}",
    minimum_transfers
  );
}

fn parse_orbit_map(path: &PathBuf) -> Result<OrbitMap> {
  let orbit_lines = read_lines(path)?;
  let mut objects_map: OrbitMap = HashMap::new();
  for line in orbit_lines {
    let orbit_str = line?;
    let orbit: Vec<&str> = orbit_str.split(')').collect();

    objects_map
      .entry(orbit[0].to_string())
      .or_insert_with(|| None);

    objects_map.insert(orbit[1].to_string(), Some(orbit[0].to_string()));
  }

  Ok(objects_map)
}

fn orbit_count_checksum(map: &OrbitMap) -> u32 {
  map
    .keys()
    .map(|object| path_to_com(object, map).len() as u32)
    .sum()
}

/// Does not include the object itself
fn path_to_com<'a>(object: &str, map: &'a OrbitMap) -> Vec<&'a String> {
  let mut path: Vec<&String> = vec![];
  let mut current: &Option<String> = map.get(object).unwrap();

  while current.is_some() {
    path.push(current.as_ref().unwrap());
    current = map.get(current.as_ref().unwrap()).unwrap();
  }

  path
}

fn min_orbital_transfers(start: &str, target: &str, map: &OrbitMap) -> u32 {
  let start_path: Vec<&String> = path_to_com(start, map);
  let target_path: Vec<&String> = path_to_com(target, map);

  // Reversed so both paths start from COM (meaning `take_while` will stop returning true when they diverge)
  // Not reversing would fail to find the common path length unless the paths are the same length
  let common_path_len = start_path
    .iter()
    .rev()
    .zip(target_path.iter().rev())
    .take_while(|(a, b)| a == b)
    .count();

  // The path between two objects is the sum of the path to their first common ancestor/orbit,
  // which is the same as the sum of their path to the root (COM) minus double (both paths) the common path length
  (start_path.len() + target_path.len() - 2 * common_path_len) as u32
}
