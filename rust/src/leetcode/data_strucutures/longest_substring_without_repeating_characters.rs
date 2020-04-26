// https://leetcode.com/problems/longest-repeating-character-replacement/
#[allow(dead_code)]
struct Solution {}
//-------------------------------------
use std::ops::Range;

impl Solution {
    #[allow(dead_code)]
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut last_seen_index: [i32; 128] = [-1; 128]; // array index: char as usize; i32 is used because of -ve.
        let mut max_non_repeating_length = 0;
        let mut window: Range<usize> = 0..0;

        for (i, ch) in s.char_indices() {
            window.end = i + 1; // range end exclusively
            let last_seen_i = &last_seen_index[ch as usize];
            // is last seen within this window?
            if *last_seen_i >= window.start as i32 {
                window.start = (*last_seen_i + 1) as usize;
            }
            last_seen_index[ch as usize] = i as i32;
            max_non_repeating_length = max_non_repeating_length.max(window.len());
        }
        max_non_repeating_length as i32
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        for (ans, s) in vec![
            (3, "abcabcbb"),
            (1, "bbbbb"),
            (3, "pwwkew"),
            (2, "abba"),
            (5, "tmmzuxt"),
        ] {
            assert_eq!(ans, Solution::length_of_longest_substring(s.to_string()));
        }
    }
}
