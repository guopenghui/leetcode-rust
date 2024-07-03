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
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
struct Solution;
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut new_head = Some(Box::new(ListNode { val: 0, next: head }));

        // 需要两个指针，其中一个可变引用，所以需要unsafe
        unsafe {
            let mut tail = new_head.as_ref()? as *const Box<ListNode>;
            let mut index = new_head.as_mut()? as *mut Box<ListNode>;

            for _ in 0..n {
                tail = (*tail).next.as_ref()? as *const Box<ListNode>;
            }

            while let Some(node) = (*tail).next.as_ref() {
                tail = node as *const Box<ListNode>;
                index = (*index).next.as_mut()? as *mut Box<ListNode>;
            }
            (*index).next = (*index).next.as_mut()?.next.take();
        }
        new_head.as_mut()?.next.take()
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
            Solution::remove_nth_from_end:
            (list![1,2,3,4,5], 2) => list![1,2,3,5],
            (list![1], 1) => list![],
            (list![1,2], 1) => list![1],
        }
    }
}
