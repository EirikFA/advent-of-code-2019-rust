use crate::{util::read_lines, Result};
use std::{collections::HashSet, path::PathBuf};

#[derive(Debug)]
enum Direction {
  Up(u32),
  Down(u32),
  Left(u32),
  Right(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub i32, pub i32);

fn parse_wire_path(wire: &str) -> Result<Vec<Direction>> {
  Ok(
    wire
      .split(',')
      .map(|segment| {
        let (direction, distance) = segment.split_at(1);
        let distance = distance.parse::<u32>()?;
        match direction {
          "U" => Ok(Direction::Up(distance)),
          "D" => Ok(Direction::Down(distance)),
          "L" => Ok(Direction::Left(distance)),
          "R" => Ok(Direction::Right(distance)),
          _ => return Err(format!("Invalid direction: {}", direction).into()),
        }
      })
      .collect::<Result<Vec<_>>>()?,
  )
}

fn read_wire_paths(path: &PathBuf) -> Result<(Vec<Direction>, Vec<Direction>)> {
  let mut lines = read_lines(path)?;
  let wire_1 = lines.next();
  let wire_2 = lines.next();

  if let (Some(wire_1_str), Some(wire_2_str)) = (wire_1, wire_2) {
    Ok((
      parse_wire_path(wire_1_str?.as_str())?,
      parse_wire_path(wire_2_str?.as_str())?,
    ))
  } else {
    Err("Invalid input, must be at least two lines".into())
  }
}

fn wire_points(wire: &[Direction]) -> Vec<Point> {
  let mut cords = vec![];
  let mut x = 0;
  let mut y = 0;

  for direction in wire {
    match direction {
      Direction::Up(distance) => {
        for _ in 0..*distance {
          y += 1;
          cords.push(Point(x, y));
        }
      }
      Direction::Down(distance) => {
        for _ in 0..*distance {
          y -= 1;
          cords.push(Point(x, y));
        }
      }
      Direction::Left(distance) => {
        for _ in 0..*distance {
          x -= 1;
          cords.push(Point(x, y));
        }
      }
      Direction::Right(distance) => {
        for _ in 0..*distance {
          x += 1;
          cords.push(Point(x, y));
        }
      }
    }
  }

  cords
}

fn points_intersections(wire_1_points: &Vec<Point>, wire_2_points: &Vec<Point>) -> HashSet<Point> {
  let wire_1_set: HashSet<Point> = HashSet::from_iter(wire_1_points.clone());
  let wire_2_set: HashSet<Point> = HashSet::from_iter(wire_2_points.clone());
  wire_1_set.intersection(&wire_2_set).map(|p| *p).collect()
}

// Central port/starting point is (0, 0)
pub fn manhattan_distance(x: i32, y: i32) -> u32 {
  (x.abs() + y.abs()).try_into().unwrap()
}

pub fn closest_wire_intersection() -> Result<Point> {
  let (wire_1, wire_2) = read_wire_paths(&PathBuf::from("src/input/day_3.txt"))?;
  let wire_1_points = wire_points(&wire_1);
  let wire_2_points = wire_points(&wire_2);
  let intersections = points_intersections(&wire_1_points, &wire_2_points);

  let mut closest_intersection: Option<Point> = None;
  let mut closest_distance: Option<u32> = None;

  for intersection in intersections {
    let distance = manhattan_distance(intersection.0, intersection.1);
    if distance < closest_distance.unwrap_or(u32::MAX) {
      closest_distance = Some(distance);
      closest_intersection = Some(intersection);
    }
  }

  if let Some(closest_intersection) = closest_intersection {
    Ok(closest_intersection)
  } else {
    Err("No intersections found".into())
  }
}

fn total_steps_to_intersections(wire_1: Vec<Direction>, wire_2: Vec<Direction>) -> Vec<usize> {
  let wire_1_points = wire_points(&wire_1);
  let wire_2_points = wire_points(&wire_2);
  let intersections = points_intersections(&wire_1_points, &wire_2_points);

  intersections
    .into_iter()
    .map(|intersection| {
      let wire_1_steps = wire_1_points
        .iter()
        .position(|p| *p == intersection)
        .unwrap();
      let wire_2_steps = &wire_2_points
        .iter()
        .position(|p| *p == intersection)
        .unwrap();
      // Plus two as steps start at 1 while index starts at 0
      wire_1_steps + wire_2_steps + 2
    })
    .collect()
}

pub fn lowest_steps_to_intersection() -> Result<usize> {
  let (wire_1, wire_2) = read_wire_paths(&PathBuf::from("src/input/day_3.txt"))?;
  let steps_to_intersections = total_steps_to_intersections(wire_1, wire_2);
  Ok(*steps_to_intersections.iter().min().unwrap())
}
