// https://leetcode.com/problems/validate-binary-search-tree/

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
// --------------------------------------------------------------
use std::cell::RefCell;
use std::rc::Rc;

type Shorter<T> = Rc<RefCell<T>>;
#[allow(dead_code)]
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::in_order_transversal(root.as_ref(), &mut None)
        // Self::is_valid_bst_recursive(&root, None, None)
    }

    fn in_order_transversal(root: Option<&Rc<RefCell<TreeNode>>>, prev: &mut Option<i32>) -> bool {
        if let Some(n) = root {
            let nn = n.borrow();
            if !Self::in_order_transversal(nn.left.as_ref(), prev) {
                return false;
            }
            if prev.map_or(false, |p|nn.val <= p) {
                // in-order => new_val must > prev
                return false;
            }
            *prev = Some(nn.val);
            if !Self::in_order_transversal(nn.right.as_ref(), prev) {
                return false;
            }
        }
        true
    }

    fn is_valid_bst_recursive(
        node: &Option<Shorter<TreeNode>>,
        upper_limit: Option<i32>,
        lower_limit: Option<i32>,
    ) -> bool {
        match node {
            Some(n) => {
                let v = n.borrow().val;

                if upper_limit.map_or(false, |x| v >= x) || lower_limit.map_or(false, |x| v <= x) {
                    false
                } else if !Self::is_valid_bst_recursive(&n.borrow().left, Some(v), lower_limit) {
                    false
                } else if !Self::is_valid_bst_recursive(&n.borrow().right, upper_limit, Some(v)) {
                    false
                } else {
                    true
                }
            }
            None => true,
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn it_works() {
        for (expected, input) in vec![
            (true, 
                Some(Rc::new(RefCell::new(TreeNode::new(-2147483648))))
            ),
            (true, {
                let right = Rc::new(RefCell::new(TreeNode::new(3)));
                let left = Rc::new(RefCell::new(TreeNode::new(1)));

                let mut this = TreeNode::new(2);
                this.left = Some(left);
                this.right = Some(right);
                Some(Rc::new(RefCell::new(this)))
            }),
            (false, {
                let right = Rc::new(RefCell::new(TreeNode::new(6)));
                let left = Rc::new(RefCell::new(TreeNode::new(3)));
                let mut sub_root = TreeNode::new(4);
                sub_root.right = Some(right);
                sub_root.left = Some(left);
                let left = Rc::new(RefCell::new(TreeNode::new(1)));
                let mut this = TreeNode::new(5);
                this.left = Some(left);
                this.right = Some(Rc::new(RefCell::new(sub_root)));
                Some(Rc::new(RefCell::new(this)))
            }),
            (false, {
                // [10,5,15,null,null,6,20]
                let mut a = TreeNode::new(10);
                let b = TreeNode::new(5);
                let mut c = TreeNode::new(15);
                let d = TreeNode::new(6);
                let e = TreeNode::new(20);
                c.left = Some(Rc::new(RefCell::new(d)));
                c.right = Some(Rc::new(RefCell::new(e)));
                a.left = Some(Rc::new(RefCell::new(b)));
                a.right = Some(Rc::new(RefCell::new(c)));
                Some(Rc::new(RefCell::new(a)))
            }),
            (false, {
                // [3,1,5,0,2,4,6,null,null,null,3]
                let mut a = TreeNode::new(3);
                let mut b = TreeNode::new(1);
                let mut c = TreeNode::new(5);
                let d = TreeNode::new(0);
                let mut e = TreeNode::new(2);
                let f = TreeNode::new(4);
                let g = TreeNode::new(6);
                let h = TreeNode::new(3);
                e.right = Some(Rc::new(RefCell::new(h)));
                c.left = Some(Rc::new(RefCell::new(f)));
                c.right = Some(Rc::new(RefCell::new(g)));
                b.left = Some(Rc::new(RefCell::new(d)));
                b.right = Some(Rc::new(RefCell::new(e)));
                a.left = Some(Rc::new(RefCell::new(b)));
                a.right = Some(Rc::new(RefCell::new(c)));
                Some(Rc::new(RefCell::new(a)))
            }),
        ] {
            assert_eq!(expected, Solution::is_valid_bst(input));
        }
    }
}
