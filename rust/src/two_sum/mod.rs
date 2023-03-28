use std::{vec, collections::HashMap};

pub struct Solution;

impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    for (i, a) in nums.iter().enumerate() {
      for (j, b) in nums.iter().enumerate() {
        if i != j && a+b == target {
          return vec![i as i32, j as i32];
        }
      }
    }

    return Vec::new();
  }


  pub fn two_sum_hash(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, elem) in nums.iter().enumerate() {
      if map.contains_key(elem) {
        return vec![map[elem], i as i32];
      }
      map.insert(target - elem, i as i32);
    }

    return Vec::new();
  }
}

#[cfg(test)]
mod tests {
  use two_sum::Solution;

  #[test]
  fn test1() {
    let nums = vec![2, 7, 11, 15];  let target = 9;
    let solution = Solution::two_sum_hash(nums, target);

    assert_eq!(solution, vec![0, 1]);
  }

  #[test]
  fn test2() {
    let nums = vec![3,2,4];   let target = 6;
    let solution = Solution::two_sum_hash(nums, target);

    assert_eq!(solution, vec![1, 2]);
  }

  #[test]
  fn test3() {
    let nums = vec![3, 3];  let target = 6;
    let solution = Solution::two_sum_hash(nums, target);

    assert_eq!(solution, vec![0, 1]);
  }
}