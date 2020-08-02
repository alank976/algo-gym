// https://leetcode.com/problems/binary-tree-level-order-traversal/

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

// -----------------
#[allow(dead_code)]
struct Solution;

use std::{cell::RefCell, collections::LinkedList, rc::Rc};
#[allow(dead_code)]
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = LinkedList::new();
        if let Some(node) = root {
            queue.push_back((node, 0));
        }
        let mut result = Vec::new();
        while let Some((node, level)) = queue.pop_front() {
            let node = node.borrow();
            if result.len() == level {
                result.push(Vec::new());
            }
            result[level].push(node.val);
            if let Some(left) = &node.left {
                queue.push_back((left.clone(), level + 1));
            }
            if let Some(right) = &node.right {
                queue.push_back((right.clone(), level + 1));
            }
        }
        result
    }

    fn bfs(queue: Vec<Rc<RefCell<TreeNode>>>, mut result: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if queue.is_empty() {
            return result;
        }
        let vals: Vec<i32> = queue.iter().map(|x| x.borrow().val).collect();
        result.push(vals);
        let next_level_nodes: Vec<_> = queue
            .iter()
            .flat_map(|x| {
                let x = x.borrow();
                vec![x.left.clone(), x.right.clone()]
            })
            .filter_map(|x| x)
            .collect();
        Self::bfs(next_level_nodes, result)
    }
}

#[cfg(test)]
mod test_non_linear_data {
    use super::*;

    #[test]
    fn test_level_order() {
        // [3,9,20,null,null,15,7]
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
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];
        assert_eq!(expected, Solution::level_order(Some(rc_ref(root))))
    }

    fn rc_ref<T>(t: T) -> Rc<RefCell<T>> {
        Rc::new(RefCell::new(t))
    }
}
