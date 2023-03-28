struct Solution;

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut a = l1.as_ref();
    let mut b = l2.as_ref();

    let mut result = ListNode::new(0);
    let mut current = &mut result;
  
    while a.is_some() && b.is_some() {

      let sum = sum(&a?.val, &b?.val);

      a = a?.next.as_ref();
      b = b?.next.as_ref();

      current.val += sum.digit;


      if a.is_some() {
        current.next = Some(Box::new(ListNode::new(sum.reminder)));
        current = current.next.as_mut()?;
      }
    }

    match (a, b) {
      (Some(_), None) => {
        while a.is_some() {
          let sum = sum(&a?.val, &current.val);

          a = a?.next.as_ref();
          current.val = sum.digit;


          current.next = Some(Box::new(ListNode::new(sum.reminder)));
          current = current.next.as_mut()?;
        }
      }
      (None, Some(_)) => {
        while b.is_some() {
          let sum = sum(&current.val, &b?.val);

          b = b?.next.as_ref();
          current.val = sum.digit;

          current.next = Some(Box::new(ListNode::new(sum.reminder)));
          current = current.next.as_mut()?;
        }
      }
      _ => {}
    }

    return Some(Box::new(result));
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
}