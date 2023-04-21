use crate::util::digits;

pub const LOWER_LIMIT: usize = 134564;
pub const UPPER_LIMIT: usize = 585159;

#[allow(dead_code)]
fn matching_adjacent_digits(num: usize) -> bool {
  let digits = digits(num);
  let mut last = &digits[0];
  for digit in digits.iter().skip(1) {
    if digit == last {
      return true;
    }
    last = digit;
  }

  false
}

fn exactly_two_matching_adjacent_digits(num: usize) -> bool {
  let digits = digits(num);
  let mut last = &digits[0];
  let mut count = 1;
  for digit in digits.iter().skip(1) {
    if digit == last {
      count += 1;
    } else {
      if count == 2 {
        return true;
      }
      count = 1;
    }
    last = digit;
  }

  count == 2
}

fn digits_do_not_decrease(num: usize) -> bool {
  let digits = digits(num);
  let mut last = &digits[0];
  for digit in digits.iter().skip(1) {
    if digit < last {
      return false;
    }
    last = digit;
  }

  true
}

fn meets_criteria(num: usize) -> bool {
  exactly_two_matching_adjacent_digits(num) && digits_do_not_decrease(num)
}

pub fn possible_passwords_count() -> usize {
  (LOWER_LIMIT..=UPPER_LIMIT)
    .filter(|num| meets_criteria(*num))
    .count()
}
