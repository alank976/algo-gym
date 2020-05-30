// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/

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
//---------------------------
use core::ops::Range;
use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        None
    }

    fn split_and_cache_inorder_approach(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let index_by_value_inorder =
            inorder
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut m, (i, &x)| {
                    m.insert(x, i);
                    m
                });
        None
    }

    fn split(
        pre_range: Range<usize>,
        in_range: Range<usize>,
        preorder: Vec<i32>,
        inorder: Vec<i32>,
        mut index_by_value_inorder: HashMap<i32, usize>,
    ) {
        let first_value = preorder[pre_range.start];
        let index_in_inorder = index_by_value_inorder
            .remove(&first_value)
            .expect("inorder and preorder values mismatched");
        let left_in = in_range.start..index_in_inorder;
        let left_pre = pre_range.start + 1..pre_range.start + 1 + left_in.len();
        split()
        
        let right_in = index_in_inorder..in_range.end;

    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_it_works() {
        // preorder = [3,9,20,15,7]
        // inorder = [9,3,15,20,7]
        /*
            3
           / \
           9  20
             /  \
            15   7
        */
        let root = {
            let mut n = TreeNode::new(3);
            n.left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
            n.right = Some(Rc::new(RefCell::new({
                let mut nn = TreeNode::new(20);
                nn.left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
                nn.right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
                nn
            })));
            n
        };
        assert_eq!(
            Some(Rc::new(RefCell::new(root))),
            Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7])
        );
    }
}
