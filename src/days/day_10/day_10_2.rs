use super::{best_location, find_reachable, AsteroidMap, Position};

pub fn run_day_10_2(asteroids: &mut AsteroidMap) {
  let (_, station) = best_location(asteroids).unwrap();
  let mut by_angle: Vec<Position> = reachable_by_angle(&station, asteroids);

  let mut last_destroyed = Position { x: 0, y: 0 };
  for _ in 1..=200 {
    if by_angle.len() == 0 {
      by_angle = reachable_by_angle(&station, asteroids);
    }

    last_destroyed = by_angle.remove(0);
    asteroids.remove(&last_destroyed);
  }

  println!(
    "INCORRECT -- Day 10, part 2 - 200th asteroid destroyed: {}",
    last_destroyed.x * 100 + last_destroyed.y
  );
}

fn reachable_by_angle(station: &Position, asteroids: &AsteroidMap) -> Vec<Position> {
  let reachable = find_reachable(station, asteroids);
  sort_asteroids_by_angle(station, &reachable)
}

fn sort_asteroids_by_angle(station: &Position, asteroids: &AsteroidMap) -> Vec<Position> {
  let mut angles: Vec<(&Position, f32)> = Vec::new();
  for asteroid in asteroids {
    let diff = *asteroid - *station;
    let mut angle = f32::atan2(diff.x as f32, diff.y as f32);
    if angle < 0.0 {
      angle += 2.0 * std::f32::consts::PI;
    }
    angles.push((asteroid, angle));
  }

  angles.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
  angles.iter().map(|a| a.0.clone()).collect()
}
