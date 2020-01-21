struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut index_by_char_arr: [i32; 128] = [-1; 128];
        let mut start: i32 = 0;
        let mut max_non_repeating_length: i32 = 0;

        for (i, ch) in s.char_indices() {
            let last_seen_i = &index_by_char_arr[ch as usize];
            if last_seen_i >= &start {
                start = *last_seen_i + 1;
            }
            index_by_char_arr[ch as usize] = i as i32;
            let current_length = i as i32 - start + 1;
            if current_length > max_non_repeating_length {
                max_non_repeating_length = current_length;
            }
        }
        max_non_repeating_length
    }
}

#[cfg(test)]
mod tests {

    use crate::longest_substring_without_repeating_characters::Solution;

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
