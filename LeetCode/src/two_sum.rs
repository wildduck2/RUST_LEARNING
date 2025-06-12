use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();

    for (idx, num) in nums.iter().enumerate() {
      /*
      NOTE: to get 0ms i just removed the variable here
      and moved down and use it's reference
      what i am doing' here i am getting the complemnt of the subtraction and
      if it's a number i have in the seem map return it
      if not i set the number with the index as value
      and move forward till i find the other pair

      vec![1,2,4,5] => target 3

      |  3 - 1 = 2 !   |
      |  3 - 2 = 1 yes |
       \  vec![1, 2] /
      */
      if let Some(&j) = seen.get(&(target - num)) {
        return vec![j, idx as i32];
      } else {
        seen.insert(*num, idx as i32);
      }
    }

    vec![]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_basic_case() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
  }

  #[test]
  fn test_unsorted_input() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![1, 2]);
  }

  #[test]
  fn test_with_duplicates() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![0, 1]);
  }

  #[test]
  fn test_negative_numbers() {
    let nums = vec![-1, -2, -3, -4, -5];
    let target = -8;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![2, 4]);
  }

  #[test]
  fn test_large_input() {
    let nums = (1..=10_000).collect::<Vec<i32>>();
    let target = 19999;
    let result = Solution::two_sum(nums, target);
    assert_eq!(result, vec![9998, 9999]);
  }
}
