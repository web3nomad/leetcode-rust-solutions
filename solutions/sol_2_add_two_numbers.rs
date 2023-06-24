pub struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // add two numbers represented by two linked lists
        // return the sum as a linked list
        let mut p1 = l1;
        let mut p2 = l2;
        let mut carry = 0;
        let mut root = Some(Box::<ListNode>::new(ListNode::new(carry)));
        // let mut psum = root.unwrap();
        let mut psum = root.as_mut().unwrap();
        loop {
            if let Some(node1) = p1 {
                psum.val += node1.val;
                p1 = node1.next;
            }
            if let Some(node2) = p2 {
                psum.val += node2.val;
                p2 = node2.next;
            }
            carry = psum.val / 10;
            psum.val %= 10;
            if p1.is_some() || p2.is_some() || carry > 0 {
                psum.next = Some(Box::<ListNode>::new(ListNode::new(carry)));
                psum = psum.next.as_mut().unwrap();
            } else {
                break;
            }
        }
        root
    }
}
// @lc code=end


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let l1 = Some(Box::<ListNode>::new(ListNode::new(2)));
        let l1 = Some(Box::<ListNode>::new(ListNode{val: 4, next: l1}));
        let l1 = Some(Box::<ListNode>::new(ListNode{val: 3, next: l1}));
        let l2 = Some(Box::<ListNode>::new(ListNode::new(5)));
        let l2 = Some(Box::<ListNode>::new(ListNode{val: 6, next: l2}));
        let l2 = Some(Box::<ListNode>::new(ListNode{val: 4, next: l2}));
        let lsum = Solution::add_two_numbers(l1, l2);
        let mut psum = lsum;
        let mut sum = 0;
        let mut base = 1;
        while let Some(node) = psum {
            sum += node.val * base;
            base *= 10;
            psum = node.next;
        }
        assert_eq!(sum, 807);
    }
}
