pub struct Solution;
impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut vec: Vec<char> = Vec::new();

    for c in s.chars() {
      if c == '(' || c == '[' || c == '{' {
        vec.push(c);
        continue;
      }

      if s.len() % 2 == 0 && vec.len() > 0 {
        if (c == ')' && vec[vec.len() - 1] == '(')
          || (c == ']' && vec[vec.len() - 1] == '[')
          || (c == '}' && vec[vec.len() - 1] == '{')
        {
          vec.pop();
        } else {
          return false;
        }
      } else {
        return false;
      }
    }
    println!("{:?}", vec);

    if vec.len() != 0 {
      return false;
    } else {
      return true;
    }
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty_string() {
    assert_eq!(Solution::is_valid("".to_string()), true);
  }

  #[test]
  fn test_single_pair() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("[]".to_string()), true);
    assert_eq!(Solution::is_valid("{}".to_string()), true);
  }

  #[test]
  fn test_nested_pairs() {
    assert_eq!(Solution::is_valid("({[]})".to_string()), true);
  }

  #[test]
  fn test_unbalanced() {
    assert_eq!(Solution::is_valid("(".to_string()), false);
    assert_eq!(Solution::is_valid("]".to_string()), false);
    assert_eq!(Solution::is_valid("({[)]}".to_string()), false);
  }

  #[test]
  fn test_wrong_order() {
    assert_eq!(Solution::is_valid(")(".to_string()), false);
    assert_eq!(Solution::is_valid("{[}]".to_string()), false);
  }

  #[test]
  fn test_long_valid() {
    assert_eq!(Solution::is_valid("()[]{}".repeat(100).to_string()), true);
  }

  #[test]
  fn test_long_invalid() {
    let mut s = "(".repeat(100);
    s.push(']');
    assert_eq!(Solution::is_valid(s), false);
  }
}
