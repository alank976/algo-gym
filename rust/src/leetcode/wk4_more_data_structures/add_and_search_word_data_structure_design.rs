// https://leetcode.com/problems/add-and-search-word-data-structure-design/

use std::cell::RefCell;
use std::collections::HashMap;
use std::{rc::Rc, str::Chars};

struct WordDictionary {
    root: Rc<RefCell<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    /** Initialize your data structure here. */
    fn new() -> Self {
        Self {
            root: Node::new_rc_ref(),
        }
    }
    /** Adds a word into the data structure. */
    fn add_word(&self, word: String) {
        word.chars()
            .fold(self.root.clone(), |n, ch| {
                n.borrow_mut()
                    .children
                    .entry(ch)
                    .or_insert(Node::new_rc_ref())
                    .clone()
            })
            .borrow_mut()
            .exist = true;
    }
    /** Returns if the word is in the data structure. A word could contain the dot character '.' to represent any one letter. */
    fn search(&self, word: String) -> bool {
        Self::search_node(word.chars(), vec![self.root.clone()])
    }

    fn search_node(mut chars: Chars, nodes: Vec<Rc<RefCell<Node>>>) -> bool {
        match chars.next() {
            Some(ch) => {
                let next_nodes: Vec<Rc<RefCell<Node>>> = if ch != '.' {
                    nodes
                        .into_iter()
                        .filter_map(|n| n.borrow().find_children(&ch))
                        .collect()
                } else {
                    nodes
                        .into_iter()
                        .flat_map(|n| n.borrow().all_children())
                        .collect()
                };
                if next_nodes.is_empty() {
                    false
                } else {
                    Self::search_node(chars, next_nodes)
                }
            }
            None => nodes.into_iter().any(|n| n.borrow().exist),
        }
    }
}

struct Node {
    children: HashMap<char, Rc<RefCell<Node>>>,
    exist: bool,
}

impl Node {
    fn new() -> Self {
        Node {
            children: HashMap::new(),
            exist: false,
        }
    }

    fn new_rc_ref() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new()))
    }

    fn find_children(&self, ch: &char) -> Option<Rc<RefCell<Node>>> {
        self.children.get(ch).cloned()
    }

    fn all_children(&self) -> Vec<Rc<RefCell<Node>>> {
        self.children.values().cloned().collect()
    }
}

#[cfg(test)]
mod test_more_data_structures {
    use super::*;

    #[test]
    fn test_word_dict() {
        let obj = WordDictionary::new();
        obj.add_word("bad".to_owned());
        obj.add_word("dad".to_owned());
        obj.add_word("mad".to_owned());
        assert_eq!(false, obj.search("pad".to_owned()));
        assert_eq!(true, obj.search("bad".to_owned()));
        assert_eq!(true, obj.search(".ad".to_owned()));
        assert_eq!(true, obj.search("b..".to_owned()));

        assert_eq!(false, obj.search("d.e".to_owned()));
    }

    #[test]
    fn failed_test() {
        // ["WordDictionary","addWord","addWord","addWord","addWord","search","search","addWord","search","search","search","search","search","search"]
        // [[],["at"],["and"],["an"],["add"],["a"],[".at"],["bat"],[".at"],["an."],["a.d."],["b."],["a.d"],["."]]
        // [null,null,null,null,null,false,false,null,true,true,false,false,true,false]
        let obj = WordDictionary::new();
        obj.add_word("at".to_owned());
        obj.add_word("and".to_owned());
        obj.add_word("an".to_owned());
        obj.add_word("add".to_owned());
        assert_eq!(false, obj.search("a".to_owned()));
        assert_eq!(false, obj.search(".at".to_owned()));

        obj.add_word("bat".to_owned());
        assert_eq!(true, obj.search(".at".to_owned()));
        assert_eq!(true, obj.search("an.".to_owned()));

        assert_eq!(false, obj.search("a.d.".to_owned()));
        assert_eq!(false, obj.search("b.".to_owned()));
        assert_eq!(true, obj.search("a.d".to_owned()));
        assert_eq!(false, obj.search(".".to_owned()));
    }
}
