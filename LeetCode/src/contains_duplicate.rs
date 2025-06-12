use std::collections::HashSet;

pub struct Solution {}

//TEST: this will be 1ms
impl Solution {
  pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.iter().collect::<HashSet<_>>().len() != nums.len()
  }
}

// // NOTE: THIS solution is 6ms speed on leetcode
// impl Solution {
//   pub fn contains_duplicate(nums: Vec<i32>) -> bool {
//     let mut nums_map: HashSet<i32> = HashSet::new();
//     let mut has_dub = false;
//
//     for number in nums.iter() {
//       match nums_map.get(number) {
//         Some(_) => {
//           has_dub = true;
//           break;
//         }
//         _ => {
//           nums_map.insert(*number);
//           has_dub = false
//         }
//       };
//     }
//
//     has_dub
//   }
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_contains_duplicate_basic() {
    let result = Solution::contains_duplicate(vec![1, 2, 3, 1]);
    assert_eq!(result, true);
  }

  #[test]
  fn test_contains_duplicate_no_duplicates() {
    let result = Solution::contains_duplicate(vec![1, 2, 3]);
    assert_eq!(result, false);
  }

  #[test]
  fn test_contains_duplicate_with_zeroes() {
    let result = Solution::contains_duplicate(vec![0, 4, 5, 0, 3, 6]);
    assert_eq!(result, true);
  }

  #[test]
  fn test_contains_duplicate_empty() {
    let result = Solution::contains_duplicate(vec![]);
    assert_eq!(result, false);
  }

  #[test]
  fn test_contains_duplicate_single_element() {
    let result = Solution::contains_duplicate(vec![42]);
    assert_eq!(result, false);
  }

  #[test]
  fn test_contains_duplicate_all_same() {
    let result = Solution::contains_duplicate(vec![7, 7, 7, 7, 7]);
    assert_eq!(result, true);
  }

  #[test]
  fn test_contains_duplicate_large_unique() {
    let result = Solution::contains_duplicate((0..10_000).collect());
    assert_eq!(result, false);
  }

  #[test]
  fn test_contains_duplicate_large_with_duplicate() {
    let mut v: Vec<i32> = (0..10_000).collect();
    v.push(9999);
    let result = Solution::contains_duplicate(v);
    assert_eq!(result, true);
  }
}
