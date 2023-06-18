use crate::{util::read_lines, Result};
use num_integer::gcd;
use std::{
  collections::HashSet,
  ops::{Div, Sub},
  path::PathBuf,
};

use self::day_10_2::run_day_10_2;

mod day_10_2;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
  x: isize,
  y: isize,
}

impl Sub for Position {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}

impl Div<isize> for Position {
  type Output = Self;

  fn div(self, divisor: isize) -> Self::Output {
    Self {
      x: self.x / divisor,
      y: self.y / divisor,
    }
  }
}

type AsteroidMap = HashSet<Position>;

pub fn run_day_10() {
  let mut asteroids = parse_asteroid_map(&PathBuf::from("src/input/day_10.txt")).unwrap();
  run_day_10_1(&asteroids);
  run_day_10_2(&mut asteroids);
}

fn run_day_10_1(asteroids: &AsteroidMap) {
  let (max_reachable, _) = best_location(asteroids).unwrap();
  println!(
    "Day 10, part 1 - most asteroids detected: {}",
    max_reachable
  );
}

fn parse_asteroid_map(path: &PathBuf) -> Result<AsteroidMap> {
  let lines = read_lines(path)?;
  let mut asteroids: AsteroidMap = HashSet::new();
  for (y, line) in lines.enumerate() {
    for (x, c) in line?.char_indices() {
      if c == '#' {
        asteroids.insert(Position {
          x: x as isize,
          y: y as isize,
        });
      }
    }
  }

  Ok(asteroids)
}

fn find_reachable(from_pos: &Position, asteroids: &AsteroidMap) -> AsteroidMap {
  let mut min_vecs: HashSet<Position> = HashSet::new();
  let mut reachable: AsteroidMap = HashSet::new();

  for target in asteroids {
    if target == from_pos {
      continue;
    }

    let diff = *target - *from_pos;
    let min_vec = diff / gcd(diff.x, diff.y);
    if !min_vecs.contains(&min_vec) {
      min_vecs.insert(min_vec);
      reachable.insert(target.clone());
    }
  }

  reachable
}

fn best_location(asteroids: &AsteroidMap) -> Option<(usize, Position)> {
  let mut max_reachable = 0;
  let mut best_location: Option<Position> = None;

  for asteroid in asteroids {
    let reachable = find_reachable(asteroid, asteroids);
    if reachable.len() > max_reachable {
      max_reachable = reachable.len();
      best_location = Some(*asteroid);
    }
  }

  best_location.and_then(|pos| Some((max_reachable, pos)))
}
