mod contains_duplicate;
mod group_anagrams;
mod top_k_frequent;
mod two_sum;
mod valid_anagram;

fn main() {
  let nums = vec![-1, -1, -2, -2, -2, -3];
  let k = 2;
  let result = top_k_frequent::Solution::top_k_frequent(nums, k);
  assert_eq!(sort_unordered_vec(result), vec![-2, -1]);
}

fn sort_unordered_vec(mut v: Vec<i32>) -> Vec<i32> {
  v.sort();
  v
}
