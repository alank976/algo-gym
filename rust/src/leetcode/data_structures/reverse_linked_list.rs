// https://leetcode.com/problems/reverse-linked-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution {}
// -----------------------------------------
use std::mem;

type Link = Option<Box<ListNode>>;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // Self::iteratively(head)
        Self::recursively(head, None)
    }

    fn iteratively(head: Link) -> Link {
        let mut current_node = head;
        let mut prev: Link = None;
        while let Some(mut inner_current_node) = current_node {
            let next = inner_current_node.next;
            inner_current_node.next = prev;
            prev = Some(inner_current_node);
            current_node = next;
        }
        prev
    }

    fn recursively(current: Link, prev: Link) -> Link {
        if let Some(mut inner_current) = current {
            let next = mem::replace(&mut inner_current.next, prev);
            Self::recursively(next, Some(inner_current))
        } else {
            prev
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = ListNode::new(5);
        let b = ListNode::new(6);
        a.next = Some(Box::new(b));
        let result = Solution::iteratively(Some(Box::new(a)));
        if let Some(r) = result {
            assert_eq!(6, r.val);
            if let Some(r2) = r.next {
                assert_eq!(5, r2.val);
            }
        }
    }
}
