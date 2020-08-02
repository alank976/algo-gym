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
        let (_, max) = Self::recursive(root);
        max.expect("at least one node")
    }

    // mentality is popping max_branch and local_max to upper node
    fn recursive(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<i32>, Option<i32>) {
        match node {
            Some(node) => {
                let node = node.borrow();
                let val = node.val;
                let (left_branch, left_local_max) = Solution::recursive(node.left.clone());
                let (right_branch, right_local_max) = Solution::recursive(node.right.clone());
                let max_branch = left_branch
                    .max(right_branch)
                    .map(|mb| mb + val)
                    .max(Some(val));
                // can disconnect branch thus carry the local_max to upper node to compare
                let local_max = max_branch
                    .max(left_local_max)
                    .max(right_local_max)
                    .max(Some(
                        left_branch.unwrap_or(0) + val + right_branch.unwrap_or(0),
                    ));
                (max_branch, local_max)
            }
            None => (None, None),
        }
    }
}

#[cfg(test)]
mod test_non_linear_data {
    use super::*;

    #[test]
    fn test_max_path_sum_when_simply_sum_of_all() {
        let root = {
            let mut n = TreeNode::new(1);
            n.left = Some(rc_ref(TreeNode::new(2)));
            n.right = Some(rc_ref(TreeNode::new(3)));
            n
        };
        assert_eq!(6, Solution::max_path_sum(Some(rc_ref(root))));
    }

    #[test]
    fn test_max_path_sum_when_path_not_include_root() {
        let root = {
            let mut n = TreeNode::new(-10);
            n.left = Some(rc_ref(TreeNode::new(9)));
            n.right = Some(rc_ref({
                let mut n_right = TreeNode::new(20);
                n_right.left = Some(rc_ref(TreeNode::new(15)));
                n_right.right = Some(rc_ref(TreeNode::new(7)));
                n_right
            }));
            n
        };
        assert_eq!(42, Solution::max_path_sum(Some(rc_ref(root))));
    }

    #[test]
    fn test_max_path_sum_when_a_greatest_leave_with_negative_parent() {
        let root = {
            let mut n = TreeNode::new(1);
            n.left = Some(rc_ref(TreeNode::new(9)));
            n.right = Some(rc_ref({
                let mut n_right = TreeNode::new(-20);
                // n_right.left = Some(rc_ref(TreeNode::new(15)));
                n_right.right = Some(rc_ref(TreeNode::new(100)));
                n_right
            }));
            n
        };
        assert_eq!(100, Solution::max_path_sum(Some(rc_ref(root))));
    }

    #[test]
    fn test_max_path_sum_when_complex() {
        let root = {
            let mut n = TreeNode::new(5);
            n.left = Some(rc_ref({
                let mut n_4 = TreeNode::new(4);
                n_4.left = Some(rc_ref({
                    let mut n_11 = TreeNode::new(11);
                    n_11.left = Some(rc_ref(TreeNode::new(7)));
                    n_11.right = Some(rc_ref(TreeNode::new(2)));
                    n_11
                }));
                n_4
            }));
            n.right = Some(rc_ref({
                let mut n_8 = TreeNode::new(8);
                n_8.left = Some(rc_ref(TreeNode::new(13)));
                n_8.right = Some(rc_ref({
                    let mut n_4 = TreeNode::new(4);
                    n_4.right = Some(rc_ref(TreeNode::new(1)));
                    n_4
                }));
                n_8
            }));
            n
        };
        assert_eq!(48, Solution::max_path_sum(Some(rc_ref(root))));
    }

    fn rc_ref<T>(t: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(t))
    }
}
