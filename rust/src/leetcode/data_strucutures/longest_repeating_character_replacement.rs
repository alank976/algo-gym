use std::{cmp::max};

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        Self::sliding_window(s, k)
    }

    fn sliding_window(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut counts: [usize; 26] = [0; 26];
        let mut max_count_ever_of_any_char_within_window = 0usize;
        let mut max_length = 0;
        let mut window = 0..0;

        for (i, c) in chars.iter().enumerate() {
            window.end = i + 1; // because it's exclusive
            let count = &mut counts[char_to_index(*c)];
            *count += 1;
            max_count_ever_of_any_char_within_window =
                max(max_count_ever_of_any_char_within_window, *count);
            while window.len() - max_count_ever_of_any_char_within_window > k as usize {
                let c = &chars[window.start];
                let char_at_start = &mut counts[char_to_index(*c)];
                *char_at_start -= 1;
                window.start += 1;
            }
            max_length = max(max_length, window.len());
        }
        max_length as i32
    }
}

fn char_to_index(c: char) -> usize {
    c as usize - 'A' as usize
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        for (expected, (s, k)) in vec![
            (4, ("ABAB".to_string(), 2)),
            (4, ("AABABBA".to_string(), 1)),
        ] {
            assert_eq!(expected, Solution::character_replacement(s, k));
        }
    }
}
