use std::collections::HashSet;

pub struct Solution;
impl Solution {
  // Use hashMaps to solve this
  // O(n)
  pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut map: Vec<HashSet<i32>> = Vec::new();
    let mut idx = 0;
    map.push(HashSet::new());

    let mut numss = nums.clone();
    numss.sort();

    for (i, n) in numss.iter().enumerate() {
      if i > 0 && numss[i - 1] + 1 != *n && numss[i - 1] != *n {
        map.push(HashSet::new());
        idx += 1;
      }

      map[idx].insert(*n);
    }

    map.iter().map(|h| h.len()).max().unwrap_or(0) as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_empty() {
    assert_eq!(Solution::longest_consecutive(vec![]), 0);
  }

  #[test]
  fn test_single_element() {
    assert_eq!(Solution::longest_consecutive(vec![100]), 1);
  }

  #[test]
  fn test_no_consecutive() {
    assert_eq!(Solution::longest_consecutive(vec![10, 30, 20]), 1);
  }

  #[test]
  fn test_simple_case() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4); // sequence: [1,2,3,4]
  }

  #[test]
  fn test_duplicates() {
    assert_eq!(Solution::longest_consecutive(vec![1, 2, 2, 3]), 3); // sequence: [1,2,3]
  }

  #[test]
  fn test_unsorted_input() {
    assert_eq!(
      Solution::longest_consecutive(vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6]),
      7
    ); // sequence: [-1,0,1,3,4,5,6,7,8,9]
  }

  #[test]
  fn test_negative_numbers() {
    assert_eq!(Solution::longest_consecutive(vec![-2, -1, 0, 1]), 4); // sequence: [-2,-1,0,1]
  }

  // #[test]
  // fn test_long_sequence() {
  //   let mut input = (1..=1000).collect::<Vec<_>>();
  //   input.shuffle(&mut rand::thread_rng());
  //   assert_eq!(Solution::longest_consecutive(input), 1000);
  // }
}
