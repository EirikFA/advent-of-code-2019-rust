use crate::days::day_8::space_image::SpaceImage;
use crate::Result;
use std::path::PathBuf;

use self::space_image::Color;

mod space_image;

pub fn run_day_8() {
  let image = SpaceImage::from_file(&PathBuf::from("src/input/day_8.txt"), 25, 6).unwrap();
  run_day_8_1(&image);
  run_day_8_2(&image);
}

fn run_day_8_1(image: &SpaceImage) {
  let layer: Vec<Color> = find_layer_fewest_zeroes(&image).unwrap();
  let ones = layer.iter().filter(|&pixel| pixel == &Color::White).count();
  let twos = layer
    .iter()
    .filter(|&pixel| pixel == &Color::Transparent)
    .count();
  println!("Day 8, Part 1: {}", ones * twos);
}

fn run_day_8_2(image: &SpaceImage) {
  image.render();
}

fn find_layer_fewest_zeroes(image: &SpaceImage) -> Result<Vec<Color>> {
  let mut fewest_zeroes = image.width * image.height + 1;
  let mut fewest_zeroes_layer: Option<Vec<Color>> = None;

  for layer in &image.layers {
    let zeroes = layer
      .data
      .iter()
      .filter(|&pixel| pixel == &Color::Black)
      .count();
    if zeroes < fewest_zeroes {
      fewest_zeroes = zeroes;
      fewest_zeroes_layer = Some(layer.data.clone());
    }
  }

  match fewest_zeroes_layer {
    Some(layer) => Ok(layer),
    None => Err("No layer found with less than maximum possible zeroes".into()),
  }
}
