pub struct Solution;

impl Solution {
  pub fn reverse(x: i32) -> i32 {
    let base = String::from(format!("{}", x));
    let mut str = String::from("");

    for c in base.chars().into_iter() {
      if c != '-' {
        str = format!("{}", c) + &str;
      }
    }

    if x.is_negative() {
      str = String::from('-') + &str;
    }

    str.parse().unwrap_or(0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reverse_positive() {
    assert_eq!(Solution::reverse(123), 321);
  }

  #[test]
  fn test_reverse_negative() {
    assert_eq!(Solution::reverse(-123), -321);
  }

  #[test]
  fn test_reverse_with_zero() {
    assert_eq!(Solution::reverse(120), 21); // trailing zero is removed
  }

  #[test]
  fn test_reverse_single_digit() {
    assert_eq!(Solution::reverse(5), 5);
  }

  #[test]
  fn test_reverse_zero() {
    assert_eq!(Solution::reverse(0), 0);
  }

  #[test]
  fn test_reverse_negative_single_digit() {
    assert_eq!(Solution::reverse(-5), -5);
  }

  #[test]
  fn test_reverse_overflow_positive() {
    assert_eq!(Solution::reverse(1534236469), 0); // would overflow
  }

  #[test]
  fn test_reverse_overflow_negative() {
    assert_eq!(Solution::reverse(-1563847412), 0); // would overflow
  }

  #[test]
  fn test_reverse_max_i32() {
    assert_eq!(Solution::reverse(2147483647), 0); // would overflow
  }

  #[test]
  fn test_reverse_min_i32() {
    assert_eq!(Solution::reverse(-2147483648), 0); // would overflow
  }
}
