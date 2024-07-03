#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

/*
 * @lc app=leetcode.cn id=2 lang=rust
 *
 * [2] 两数相加
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
struct Solution;
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut res = None;
        let mut tail = &mut res;

        let mut l1 = &l1;
        let mut l2 = &l2;

        let mut carry = 0;
        while l1.is_some() || l2.is_some() || carry == 1 {
            let mut sum = 0;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = &node.next;
            }
            if let Some(node) = l2 {
                sum += node.val;
                l2 = &node.next;
            }
            sum += carry;

            if sum > 9 {
                carry = 1;
                sum -= 10;
            } else {
                carry = 0;
            }

            let node = ListNode::new(sum);
            *tail = Some(Box::new(node));
            tail = &mut tail.as_mut().unwrap().next;
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    use crate::{batch_test, list};

    #[test]
    fn base_test() {
        batch_test! {
            Solution::add_two_numbers:
            (list![2, 4, 3], list![5, 6, 4]) => list!(7, 0, 8),
            (list![0], list![0]) => list![0],
            (list![9, 9, 9, 9, 9, 9, 9], list![9, 9, 9, 9]) => list![8, 9, 9, 9, 0, 0, 0, 1],
        }
    }

    #[test]
    fn foo() {
        assert!(Some(5) > None);
    }
}
