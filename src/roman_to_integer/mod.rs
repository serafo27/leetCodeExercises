struct Solution;

impl Solution {
  pub fn roman_to_int(s: String) -> i32 {

    if s.len() == 1 {
      return Symbols::parse(s.chars().nth(0).unwrap()) as i32
    }

    let mut prev_symbol = Symbols::None;
    let mut result: i32 = 0;

    for (idx, char) in s.char_indices() {
      let symbol = Symbols::parse(char);
      if prev_symbol == Symbols::None {
        prev_symbol = symbol;
      } else {
        let prev_symbol_value = prev_symbol.clone() as i32;
  
        if prev_symbol >= symbol { 
          result += prev_symbol_value;
        } else { 
          result -= prev_symbol_value;
        }

        if idx == s.len()-1 {
          result += symbol as i32;
        } else {
          prev_symbol = symbol;
        }
      }


    }

    return result;
  }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Symbols {
  I = 1,
  V = 5,
  X = 10,
  L = 50,
  C = 100,
  D = 500,
  M = 1000,
  None = -1
}

impl Symbols {
  fn parse(char: char) -> Symbols {
    match char {
      'I' => Symbols::I,
      'V' => Symbols::V,
      'X' => Symbols::X,
      'L' => Symbols::L,
      'C' => Symbols::C,
      'D' => Symbols::D,
      'M' => Symbols::M,
      _ => panic!("not valid roman char")
    }
  }
}

#[cfg(test)]
mod tests {
  use roman_to_integer::Solution;

  #[test]
  fn test1() {
    let solution = Solution::roman_to_int(String::from("III"));

    assert_eq!(solution, 3);
  }
}