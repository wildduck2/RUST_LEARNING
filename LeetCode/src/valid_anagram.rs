use std::collections::{HashMap, HashSet};
pub struct Solution {}

// NOTE: this will perform 1ms
impl Solution {
  pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
      return false;
    }

    let mut s_map: HashMap<char, i32> = HashMap::new();
    let mut t_map: HashMap<char, i32> = HashMap::new();

    for char in s.chars() {
      *s_map.entry(char).or_insert(0) += 1;
    }

    for char in t.chars() {
      *t_map.entry(char).or_insert(0) += 1;
    }

    t_map == s_map
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_valid_anagram() {
    // Standard true/false cases
    let result = Solution::is_anagram("anagram".to_string(), "nagaram".to_string());
    assert_eq!(result, true);

    let result = Solution::is_anagram("rat".to_string(), "car".to_string());
    assert_eq!(result, false);
  }

  #[test]
  fn test_empty_strings() {
    let result = Solution::is_anagram("".to_string(), "".to_string());
    assert_eq!(result, true);
  }

  #[test]
  fn test_different_lengths() {
    let result = Solution::is_anagram("a".to_string(), "ab".to_string());
    assert_eq!(result, false);
  }

  #[test]
  fn test_case_sensitivity() {
    let result = Solution::is_anagram("Listen".to_string(), "Silent".to_string());
    assert_eq!(result, false); // case-sensitive
  }

  #[test]
  fn test_unicode_characters() {
    let result = Solution::is_anagram("åäö".to_string(), "öäå".to_string());
    assert_eq!(result, true);
  }
}
