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
use std::{cell::RefCell, collections::HashMap, iter::Peekable, rc::Rc, vec::IntoIter};

#[allow(dead_code)]
impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        // Self::split_and_cache_inorder_approach(preorder, inorder)
        Self::two_pointer_approach(preorder, inorder)
    }

    fn two_pointer_approach(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre_iter = preorder.into_iter().peekable();
        let mut in_iter = inorder.into_iter().peekable();
        Self::build(&mut pre_iter, &mut in_iter, None)
    }

    fn build(
        preorder: &mut Peekable<IntoIter<i32>>,
        inorder: &mut Peekable<IntoIter<i32>>,
        stop: Option<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(s) = stop {
            if let Some(in_head) = inorder.peek() {
                if *in_head == s {
                    inorder.next();
                    return None;
                }
            }
        }
        if let Some(pre_head) = preorder.next() {
            let mut node = TreeNode::new(pre_head);
            node.left = Self::build(preorder, inorder, Some(pre_head));
            // if next in_head == previous stop => end this subTree, otw continue grow branches
            node.right = Self::build(preorder, inorder, stop);
            return Some(Rc::new(RefCell::new(node)));
        }
        return None;
    }

    fn split_and_cache_inorder_approach(
        preorder: Vec<i32>,
        inorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut index_by_value_inorder =
            inorder
                .iter()
                .enumerate()
                .fold(HashMap::new(), |mut m, (i, &x)| {
                    m.insert(x, i);
                    m
                });
        Self::split(0, 0..inorder.len(), &preorder, &mut index_by_value_inorder)
    }

    fn split(
        pre_start: usize,
        in_rng: Range<usize>,
        preorder: &Vec<i32>,
        index_by_value_inorder: &mut HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start >= preorder.len() {
            None
        } else {
            let value = preorder[pre_start];
            let in_index = index_by_value_inorder
                .remove(&value)
                .expect("inorder & preorder values mismatch!");
            let mut node = TreeNode::new(value);

            let left_in_rng = in_rng.start..in_index;
            let left_pre_start = pre_start + 1;
            let right_in_rng = in_index + 1..in_rng.end;
            let right_pre_start = pre_start + 1 + left_in_rng.len();

            node.left = match left_in_rng.len() {
                0 => None,
                1 => create_node(preorder[left_pre_start]),
                _ => Self::split(
                    left_pre_start,
                    left_in_rng,
                    preorder,
                    index_by_value_inorder,
                ),
            };

            node.right = match right_in_rng.len() {
                0 => None,
                1 => create_node(preorder[right_pre_start]),
                _ => Self::split(
                    right_pre_start,
                    right_in_rng,
                    preorder,
                    index_by_value_inorder,
                ),
            };
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

fn create_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
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

    #[test]
    fn test_imbalanced_tree() {
        // preorder = [1,2,3,4,5,6]
        // inorder = [6,5,4,3,2,1]

        let root = {
            let mut a = TreeNode::new(1);
            a.left = {
                let b = node_creation_helper(2);
                b.borrow_mut().left = {
                    let c = node_creation_helper(3);
                    c.borrow_mut().left = {
                        let d = node_creation_helper(4);
                        d.borrow_mut().left = {
                            let e = node_creation_helper(5);
                            e.borrow_mut().left = Some(node_creation_helper(6));
                            Some(e)
                        };
                        Some(d)
                    };
                    Some(c)
                };
                Some(b)
            };
            a
        };
        assert_eq!(
            Some(Rc::new(RefCell::new(root))),
            Solution::build_tree(vec![1, 2, 3, 4, 5, 6], vec![6, 5, 4, 3, 2, 1])
        );
    }

    fn node_creation_helper(v: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(v)))
    }
}
