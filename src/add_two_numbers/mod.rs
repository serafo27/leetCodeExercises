struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut a = &l1;
    let mut b = &l2;

    let mut result = ListNode::new(0);
    let mut current_node = &mut result;

    while a.is_some() || b.is_some() {

      match (a, b) {
        (Some(a_node), Some(b_node)) => {
          
          let sum_result = sum(&a_node.val, &b_node.val);
          
          if current_node.val + sum_result.digit < 10 {
            current_node.val += sum_result.digit;
          
            if sum_result.reminder > 0 || a_node.next.is_some() || b_node.next.is_some() {
              current_node.next = Some(Box::new(ListNode::new(sum_result.reminder)));
              current_node = current_node.next.as_mut()?;
            }
          } else {
            current_node.val = current_node.val + sum_result.digit - 10;
          
            current_node.next = Some(Box::new(ListNode::new(1)));
            current_node = current_node.next.as_mut()?;
          }
          
          
          a = &a_node.next;
          b = &b_node.next;
        },
        (Some(a_node), None) => {

          let sum_result = sum(&a_node.val, &current_node.val);
          current_node.val = sum_result.digit;

          if sum_result.reminder > 0 || a_node.next.is_some() {
            current_node.next = Some(Box::new(ListNode::new(sum_result.reminder)));
            current_node = current_node.next.as_mut()?;
          }

          a = &a_node.next;
        },
        (None, Some(b_node)) => {

          let sum_result = sum(&b_node.val, &current_node.val);
          current_node.val = sum_result.digit;

          if sum_result.reminder > 0 || b_node.next.is_some() {
            current_node.next = Some(Box::new(ListNode::new(sum_result.reminder)));
            current_node = current_node.next.as_mut()?;
          }

          b = &b_node.next;
        },
        _ => {},
      }
    }

    Some(Box::new(result))
  }
}


fn sum(a: &i32, b: &i32) -> SumResult {
  let sum = a + b;
  if sum >= 10 {
    SumResult { digit: sum - 10, reminder: 1 }
  } else {
    SumResult { digit: sum, reminder: 0 }
  }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

#[derive(Debug)]
struct SumResult {
  digit: i32,
  reminder: i32
}

#[cfg(test)]
mod tests {
  use add_two_numbers::Solution;
  use crate::add_two_numbers::ListNode;

  #[test]
  fn test1() {
  
    let l1 = Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode::new(3))) }))}));
    let l2 = Some(Box::new(ListNode{val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode::new(4))) }))}));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, Some(Box::new(ListNode{val: 7, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode::new(8))) }))})));
  }

  #[test]
  fn test2() {
  
    let l1 = Some(Box::new(ListNode::new(0)));
    let l2 = Some(Box::new(ListNode::new(0)));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, Some(Box::new(ListNode::new(0))));
  }

  #[test]
  fn test3() {
  
    let l1 = Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode::new(9)))})) }))}))})) }))}));
    let l2 = Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode::new(9)))})) }))}));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, 
      Some(Box::new(ListNode{val: 8, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode{val: 0, next: Some(Box::new(ListNode::new(1)))}))})) }))}))})) }))}))
    );
  }

  #[test]
  fn test4() {
  
    let l1 = Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode::new(9))) }))}));
    let l2 = Some(Box::new(ListNode{val: 5, next: Some(Box::new(ListNode { val: 6, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode::new(9))) })) }))}));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, Some(Box::new(ListNode{val: 7, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode { val: 4, next: Some(Box::new(ListNode{val: 0, next: Some(Box::new(ListNode::new(1)))})) })) }))})));
  }

  #[test]
  fn test5() {
  
    let l1 = Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode::new(1))) }))}));
    let l2 = Some(Box::new(ListNode::new(1)));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, Some(Box::new(ListNode{val: 0, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode::new(2))) }))})));
  }

  #[test]
  fn test6() {
  
    let l1 = Some(Box::new(ListNode::new(0)));
    let l2 = Some(Box::new(ListNode { val: 7, next: Some(Box::new(ListNode::new(3))) }));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, Some(Box::new(ListNode { val: 7, next: Some(Box::new(ListNode::new(3))) })));
  }

  #[test]
  fn test7() {
  
    let l1 = Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode::new(7))) }));
    let l2 = Some(Box::new(ListNode { val: 9, next: Some(Box::new(ListNode::new(2))) }));
    let solution = Solution::add_two_numbers(l1, l2);

    assert_eq!(solution, Some(Box::new(ListNode{val: 2, next: Some(Box::new(ListNode { val: 0, next: Some(Box::new(ListNode::new(1))) }))})));
  }
}