// https://leetcode.com/problems/clone-graph/submissions/

use std::{cell::RefCell, collections::HashMap, rc::Rc};

type RcRef<T> = Rc<RefCell<T>>;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<RcRef<Node>>,
}

#[allow(dead_code)]
impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    fn clone_graph(node: Option<RcRef<Node>>) -> Option<RcRef<Node>> {
        node.map(|n| n.clone_with(&mut HashMap::new()))
    }
}

trait CloneWithSeenMap {
    fn clone_with(self, seen: &mut HashMap<i32, RcRef<Node>>) -> Self;
}

impl CloneWithSeenMap for RcRef<Node> {
    fn clone_with(self, seen: &mut HashMap<i32, RcRef<Node>>) -> RcRef<Node> {
        let val = self.borrow().val;
        match seen.get(&val) {
            Some(node_ref) => node_ref.clone(),
            None => {
                let new_node_rc_ref = Rc::new(RefCell::new(Node::new(val)));
                seen.insert(val, new_node_rc_ref.clone());
                new_node_rc_ref.borrow_mut().neighbors = self
                    .borrow()
                    .neighbors
                    .iter()
                    .map(|neighbor| neighbor.clone().clone_with(seen))
                    .collect();
                new_node_rc_ref
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        let a = Rc::new(RefCell::new(Node::new(1)));
        let b = Rc::new(RefCell::new(Node::new(2)));
        let c = Rc::new(RefCell::new(Node::new(3)));
        a.borrow_mut().neighbors.push(b.clone());
        a.borrow_mut().neighbors.push(c.clone());
        b.borrow_mut().neighbors.push(a.clone());

        {
            let ans = Solution::clone_graph(Some(a.clone())).expect("should not");
            let ans_a = ans.borrow();
            assert_eq!(1, ans_a.val);
            let neighbors_of_a: Vec<i32> = ans_a.neighbors.iter().map(|x| x.borrow().val).collect();
            assert_eq!(vec![2, 3], neighbors_of_a);
            let ans_b = ans_a.neighbors.first().expect("should not").borrow();
            let neighbors_of_b: Vec<i32> = ans_b.neighbors.iter().map(|x| x.borrow().val).collect();
            assert_eq!(vec![1], neighbors_of_b);
        }
    }
}
