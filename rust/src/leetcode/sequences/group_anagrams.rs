// https://leetcode.com/problems/group-anagrams/
#[allow(dead_code)]
struct Solution {}
// ----------------------------

use std::{collections::HashMap, str::Chars};

type CharCount = [u32; 26];

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let anagrams_by_char_count: HashMap<CharCount, Vec<String>> = strs
            .into_iter()
            .map(|s| (s.chars().group_and_count(), s))
            .fold(
                HashMap::new(),
                |mut grouped_str_by_count_map, (char_count, s)| {
                    grouped_str_by_count_map
                        .entry(char_count)
                        .and_modify(|grouped| grouped.push(s.clone()))
                        .or_insert(vec![s]);
                    grouped_str_by_count_map
                },
            );
        anagrams_by_char_count.into_iter().map(|(_, v)| v).collect()
    }
}

trait GroupAndCount<T> {
    fn group_and_count(self) -> CharCount;
}

impl GroupAndCount<char> for Chars<'_> {
    fn group_and_count(self) -> CharCount {
        self.fold([0u32; 26], |mut acc, c| {
            let i = c as usize - 'a' as usize;
            if let Some(count) = acc.get_mut(i) {
                *count += 1;
            }
            acc
        })
    }
}

// due to unordered inner string from the expectation and unordered groups, no simple unit tests...
