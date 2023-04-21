use std::{
  fs::File,
  io::{self, BufRead},
  path::Path,
};

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

pub fn digits(num: usize) -> Vec<usize> {
  let mut num = num;
  let mut digits = vec![];

  while num > 0 {
    digits.push(num % 10);
    num /= 10;
  }

  digits.reverse();
  digits
}
