use std::{fs, path::PathBuf};

use crate::Result;

#[derive(PartialEq, Clone)]
pub enum Color {
  Black,
  White,
  Transparent,
}

impl From<u8> for Color {
  fn from(digit: u8) -> Color {
    match digit {
      0 => Color::Black,
      1 => Color::White,
      2 => Color::Transparent,
      _ => panic!("Invalid color digit"),
    }
  }
}

impl Color {
  pub fn render(&self) {
    match self {
      Color::Black => print!(" "),
      Color::White => print!("█"),
      Color::Transparent => print!("?"),
    }
  }
}

pub struct SpaceImageLayer {
  pub data: Vec<Color>,
}

pub struct SpaceImage {
  pub layers: Vec<SpaceImageLayer>,
  pub width: usize,
  pub height: usize,
}

impl SpaceImage {
  pub fn from_digits(digits: Vec<u8>, width: usize, height: usize) -> Result<SpaceImage> {
    let layer_chunks = digits.chunks_exact(width * height);
    if layer_chunks.remainder().len() > 0 {
      return Err("Image data does not match dimensions".into());
    }

    let layers: Vec<SpaceImageLayer> = layer_chunks
      .map(|chunk| SpaceImageLayer {
        data: chunk
          .to_vec()
          .iter()
          .map(|pixel| Color::from(*pixel))
          .collect(),
      })
      .collect();

    Ok(SpaceImage {
      layers,
      width,
      height,
    })
  }

  pub fn from_file(path: &PathBuf, width: usize, height: usize) -> Result<SpaceImage> {
    let image_data = fs::read_to_string(path)?;
    let digits: Vec<u8> = image_data
      .trim()
      .chars()
      .map(|c| c.to_digit(10).unwrap() as u8)
      .collect();

    SpaceImage::from_digits(digits, width, height)
  }

  fn decode(&self) -> SpaceImageLayer {
    let mut decoded: Vec<Color> = vec![Color::Transparent; self.width * self.height];

    for layer in &self.layers {
      for (i, pixel) in layer.data.iter().enumerate() {
        if decoded[i] == Color::Transparent {
          decoded[i] = pixel.clone();
        }
      }
    }

    SpaceImageLayer { data: decoded }
  }

  pub fn render(&self) {
    let decoded = self.decode();
    for (i, pixel) in decoded.data.iter().enumerate() {
      if i % self.width == 0 {
        println!();
      }
      pixel.render();
    }
    println!();
  }
}
