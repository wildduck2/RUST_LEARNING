use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    for str in strs.into_iter() {
      let mut key = str.bytes().collect::<Vec<_>>();
      key.sort_unstable();

      anagrams.entry(key).or_default().push(str);
    }

    anagrams.into_values().collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sort_group(mut group: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for g in &mut group {
      g.sort(); // Sort inner lists for consistency
    }
    group.sort_by(|a, b| a.first().cmp(&b.first())); // Sort outer list
    group
  }

  #[test]
  fn test_basic_case() {
    let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
      .into_iter()
      .map(String::from)
      .collect();

    let result = Solution::group_anagrams(input);
    let expected = vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
      .into_iter()
      .map(|v| v.into_iter().map(String::from).collect())
      .collect();

    assert_eq!(sort_group(result), sort_group(expected));
  }

  #[test]
  fn test_empty_string() {
    let input = vec![""].into_iter().map(String::from).collect();
    let result = Solution::group_anagrams(input);
    let expected = vec![vec![String::from("")]];
    assert_eq!(result, expected);
  }

  #[test]
  fn test_single_letter() {
    let input = vec!["a"].into_iter().map(String::from).collect();
    let result = Solution::group_anagrams(input);
    let expected = vec![vec![String::from("a")]];
    assert_eq!(result, expected);
  }

  #[test]
  fn test_all_same_letters() {
    let input = vec!["abc", "bca", "cab", "cba", "bac", "acb"]
      .into_iter()
      .map(String::from)
      .collect();
    let result = Solution::group_anagrams(input);
    let expected = vec![vec!["abc", "bca", "cab", "cba", "bac", "acb"]
      .into_iter()
      .map(String::from)
      .collect()];
    assert_eq!(sort_group(result), sort_group(expected));
  }

  #[test]
  fn test_no_anagrams() {
    let input = vec!["one", "two", "three", "four"]
      .into_iter()
      .map(String::from)
      .collect();
    let result = Solution::group_anagrams(input);
    let expected = vec![vec!["one"], vec!["two"], vec!["three"], vec!["four"]]
      .into_iter()
      .map(|v| v.into_iter().map(String::from).collect())
      .collect();
    assert_eq!(sort_group(result), sort_group(expected));
  }
}
