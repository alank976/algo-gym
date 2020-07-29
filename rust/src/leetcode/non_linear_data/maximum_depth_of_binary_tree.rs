// https://leetcode.com/problems/maximum-depth-of-binary-tree/

// Definition for a binary tree node.
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
// ------------------------------
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let n = node.borrow();
                1 + Self::max_depth(n.left.clone()).max(Self::max_depth(n.right.clone()))
            }
        }
    }
}

#[cfg(test)]
mod test_non_linear_data {
    use super::*;

    #[test]
    fn test_max_depth() {
        //[3,9,20,null,null,15,7]
        let root = {
            let mut n = TreeNode::new(3);
            n.left = Some(rc_ref(TreeNode::new(9)));
            n.right = Some(rc_ref({
                let mut n_20 = TreeNode::new(20);
                n_20.left = Some(rc_ref(TreeNode::new(15)));
                n_20.right = Some(rc_ref(TreeNode::new(7)));
                n_20
            }));
            n
        };
        assert_eq!(3, Solution::max_depth(Some(rc_ref(root))))
    }

    fn rc_ref<T>(t: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(t))
    }
}
