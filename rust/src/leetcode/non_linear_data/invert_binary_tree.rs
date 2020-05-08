// https://leetcode.com/problems/invert-binary-tree/
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

#[allow(dead_code)]
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
#[allow(dead_code)]
struct Solution;
// -----------------------------------------------------------------------
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::swap_children(root.as_ref());
        root
    }

    fn swap_children(root: Option<&Rc<RefCell<TreeNode>>>) {
        if let Some(r) = root {
            let mut node = r.borrow_mut();
            let temp = node.left.as_ref().map(|x| x.clone());
            node.left = node.right.as_ref().map(|x| x.clone());
            node.right = temp;
            Self::swap_children(node.left.as_ref());
            Self::swap_children(node.right.as_ref());
        }
    }
}
