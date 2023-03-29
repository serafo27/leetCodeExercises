struct Solution;


impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
      return false
    }

    let digits = number_to_vec(x);

    let lastIndex = digits.len()/2;
    for i in 0..=lastIndex {

      let j = digits.len() -i -1;
      if i >= j {
        return true
      }

      let first = &digits[i];
      let last = &digits[j];

      if first != last { 
        return false 
      }
    }

    false
  }
}

fn number_to_vec(n: i32) -> Vec<i32> {
  let mut digits = Vec::new();
  let mut n = n;
  while n > 9 {
      digits.push(n % 10);
      n = n / 10;
  }
  digits.push(n);
  digits.reverse();
  digits
}

#[cfg(test)]
mod tests {
  use palindrome_numbers::Solution;

  #[test]
  fn test1() {

    assert_eq!(true, Solution::is_palindrome(-121))
  }
}