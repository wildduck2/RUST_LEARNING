use std::collections::HashMap;

pub struct Solution;

impl Solution {
  /// Input: nums = [1,2,3,4]
  /// Output: [24,12,8,6]
  pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = vec![1; n];

    let mut prefix = 1;

    for i in 0..n {
      result[i] = prefix;
      prefix *= nums[i]
    }

    let mut suffix = 1;
    for i in (0..n as i32).rev() {
      result[i as usize] *= suffix;
      suffix *= nums[i as usize]
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_product_except_self_basic() {
    let nums = vec![1, 2, 3, 4];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![24, 12, 8, 6]);
  }

  #[test]
  fn test_product_with_zeros() {
    let nums = vec![1, 2, 0, 4];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![0, 0, 8, 0]);
  }

  #[test]
  fn test_product_with_multiple_zeros() {
    let nums = vec![0, 2, 0, 4];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![0, 0, 0, 0]);
  }

  #[test]
  fn test_product_with_negatives() {
    let nums = vec![-1, 2, -3, 4];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![-24, 12, -8, 6]);
  }

  #[test]
  fn test_product_with_ones() {
    let nums = vec![1, 1, 1, 1];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![1, 1, 1, 1]);
  }

  #[test]
  fn test_product_two_elements() {
    let nums = vec![3, 4];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![4, 3]);
  }

  #[test]
  fn test_product_single_element() {
    let nums = vec![7];
    let result = Solution::product_except_self(nums);
    assert_eq!(result, vec![1]); // Convention: product of all except self = 1
  }
}
