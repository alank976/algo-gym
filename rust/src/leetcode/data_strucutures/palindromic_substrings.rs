// https://leetcode.com/problems/palindromic-substrings/
#[allow(dead_code)]
struct Solution {}
//-----------------------------------------------
#[allow(dead_code)]
impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let chars = vec!['^', '#'];
        let mut chars: Vec<char> = s.chars().fold(chars, |mut vec, ch| {
            vec.push(ch);
            vec.push('#');
            vec
        });
        chars.push('$');

        // center and rightmost of palindrome
        let (mut center, mut right) = (0, 0);
        let mut palindrome_width = vec![0usize; chars.len()];

        for i in 1..chars.len() - 1 {
            // within boundary of the palindrome, we can deduce sub-parlindrome width based on symmetry
            if i < right {
                let mirror_value = palindrome_width[center * 2 - i];
                palindrome_width[i] = mirror_value.min(right - i); // prevent it grants more than the parent parlindrome reach
            }
            while chars[i - palindrome_width[i] - 1] == chars[i + palindrome_width[i] + 1] {
                palindrome_width[i] += 1;
            }
            // update the center and rightmost of this palindrome (if any)
            if i + palindrome_width[i] > right {
                center = i;
                right = i + palindrome_width[i];
            }
        }
        palindrome_width
            .into_iter()
            .map(|w| ((w + 1) / 2) as i32)
            .sum()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    #[test]
    fn test_() {
        for (expected, input) in vec![
            (6, "abab"),
            (9, "xvxvx"),
            (
                77,
                "dnncbwoneinoplypwgbwktmvkoimcooyiwirgbxlcttgteqthcvyoueyftiwgwwxvxvg",
            ),
            (6, "hello"),
            (1, "a"),
            (8, "babba"),
            (3, "abc"),
            (6, "aaa"),
            (14, "bbaaabb"),
        ] {
            assert_eq!(expected, Solution::count_substrings(input.to_owned()));
        }
    }
}
