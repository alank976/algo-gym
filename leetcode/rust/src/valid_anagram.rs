use std::collections::HashMap;
use std::iter::Iterator;
use std::str::Chars;

struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            false
        } else {
            s.chars().group_and_count() == t.chars().group_and_count()
        }
    }
}

trait GroupAndCount<T> {
    fn group_and_count(self) -> HashMap<T, u32>;
}

impl GroupAndCount<char> for Chars<'_> {
    fn group_and_count(self) -> HashMap<char, u32> {
        self.fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|v| *v += 1).or_insert(1);
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        for (ans, s, t) in vec![(true, "anagram", "nagaram"), (false, "rat", "car")] {
            assert_eq!(ans, Solution::is_anagram(s.to_string(), t.to_string()));
        }
    }
}
