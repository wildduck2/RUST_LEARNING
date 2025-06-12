use std::collections::HashMap;

pub struct Solution;

impl Solution {
  pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut nums_map: HashMap<i32, i32> = HashMap::new();

    for num in nums.into_iter() {
      *nums_map.entry(num).or_insert(0) += 1
    }

    let mut hi: Vec<(i32, i32)> = nums_map.into_iter().collect();
    hi.sort_by(|a, b| b.1.cmp(&a.1));
    let hay: Vec<i32> = hi[0..k as usize].iter().map(|x| x.0).collect();
    hay
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn sort_unordered_vec(mut v: Vec<i32>) -> Vec<i32> {
    v.sort();
    v
  }

  #[test]
  fn test_basic_case() {
    let nums = vec![1, 1, 1, 2, 2, 3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    assert_eq!(sort_unordered_vec(result), vec![1, 2]);
  }

  #[test]
  fn test_single_element() {
    let nums = vec![1];
    let k = 1;
    let result = Solution::top_k_frequent(nums, k);
    assert_eq!(result, vec![1]);
  }

  #[test]
  fn test_all_unique() {
    let nums = vec![5, 4, 3, 2, 1];
    let k = 3;
    let result = Solution::top_k_frequent(nums, k);
    assert_eq!(result.len(), 3);
    // All numbers have frequency 1, so any 3 of them are valid
    for num in result {
      assert!(vec![1, 2, 3, 4, 5].contains(&num));
    }
  }

  #[test]
  fn test_tied_frequencies() {
    let nums = vec![4, 4, 6, 6, 7, 7, 8];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    // Three numbers have the same frequency (2), one has 1
    assert_eq!(result.len(), 2);
    for num in result {
      assert!(vec![4, 6, 7].contains(&num));
    }
  }

  #[test]
  fn test_negative_numbers() {
    let nums = vec![-1, -1, -2, -2, -2, -3];
    let k = 2;
    let result = Solution::top_k_frequent(nums, k);
    assert_eq!(sort_unordered_vec(result), vec![-2, -1]);
  }
}
