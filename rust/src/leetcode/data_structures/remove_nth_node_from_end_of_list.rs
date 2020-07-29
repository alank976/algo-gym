// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
#[allow(dead_code)]
struct Solution {}

// Definition for singly-linked list.
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
// ------------------------------------------------------------------------
#[allow(dead_code)]
/// This question in Rust sounds different to other lang due to ownership design.
/// We have to "own" the last node in order to relink (n+1)-th node to (n-1)-th (counting from the end)
/// Even it is boxed (Boxed => still single ownership of the boxed node)
/// Thus, immutable/mutable reference cannot help here so the common solution to other langs: 1 or 2 pointers wont help here
/// If there's a pointer pointing to a node, say immutable reference, then we are not able to mutate the node to relink to other node
/// If it's mutable one, then one mutable reference can be held therefore we cannot have a pointer pointing to first node that is ready for being returned.
/// Using stack is the most straght forward way with the same O(2n) time complexity
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut current = head;
        let mut stack: Vec<i32> = Vec::new();
        while let Some(curr) = current {
            stack.push(curr.val);
            current = curr.next;
        }
        let mut i = 1;
        let mut prev: Option<Box<ListNode>> = None;
        while let Some(tail_val) = stack.pop() {
            if i != n {
                let mut node = ListNode::new(tail_val);
                node.next = prev;
                prev = Some(Box::new(node));
            }
            i += 1;
        }
        prev
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let fifth = ListNode::new(5);
        let mut fourth = ListNode::new(4);
        fourth.next = Some(Box::new(fifth));
        let mut third = ListNode::new(3);
        third.next = Some(Box::new(fourth));
        let mut second = ListNode::new(2);
        second.next = Some(Box::new(third));
        let mut first = ListNode::new(1);
        first.next = Some(Box::new(second));

        let mut head = Solution::remove_nth_from_end(Some(Box::new(first)), 2);
        let mut results: Vec<i32> = Vec::new();
        while let Some(boxed_next) = head {
            results.push(boxed_next.val);
            head = boxed_next.next;
        }
        assert_eq!(vec![1, 2, 3, 5], results);
    }
}
