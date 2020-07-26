// https://leetcode.com/problems/binary-tree-maximum-path-sum/

// Definition for a binary tree node.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
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

// ---------------------------------------------------------
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        0
    }
}

#[cfg(test)]
mod test_non_linear_data {
    use super::*;

    #[test]
    fn test_max_path_sum() {
        
    }
}
