struct Solution;

impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {

    let initial = &strs[0];
    let words = &strs[1..];
    let mut solution = String::from("");

    for i in 0..initial.len() {
      let tmp_char = initial.chars().nth(i);
      if tmp_char.is_none() {
        return solution;
      }
      
      for s in words.iter() {
        let tmp_char2 = s.chars().nth(i);
        if tmp_char2.is_none() {
          return solution;
        }

        if tmp_char2.unwrap() != tmp_char.unwrap() {
          return solution;
        }
      }
      
      solution += &tmp_char.unwrap().to_string();
    }

    solution
  }
}

#[cfg(test)]
mod tests {
  use crate::longest_common_prefix::Solution;

  #[test]
  fn test1() {
    let vector = vec!["hello".to_string(), "how".to_string()];
    let solution = Solution::longest_common_prefix(vector);
    assert_eq!(solution, "h");
  }
}