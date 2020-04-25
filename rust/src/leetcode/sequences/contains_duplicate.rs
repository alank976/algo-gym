// https://leetcode.com/problems/contains-duplicate/

use std::collections::HashSet;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let approach = if nums.len() > std::usize::MAX / 2 {
            // when inputs is large enough
            Self::hash_set_approach
        } else {
            Self::sort_approach
        };
        approach(nums)
    }

    fn sort_approach(nums: Vec<i32>) -> bool {
        if nums.len() < 2 {
            return false;
        }
        let mut cloned = nums.clone();
        cloned.sort_unstable();
        let mut prev: i32 = cloned[0];
        for num in &cloned[1..] {
            if *num == prev {
                return true;
            }
            prev = *num;
        }
        false
    }

    fn hash_set_approach(nums: Vec<i32>) -> bool {
        let mut seens: HashSet<i32> = HashSet::new();
        for num in nums {
            if seens.contains(&num) {
                return true;
            }
            seens.insert(num);
        }
        false
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_it_works() {
        for (expected, inputs) in vec![
            (true, vec![1, 2, 3, 1]),
            (false, vec![1, 2, 3, 4]),
            (true, vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]),
        ] {
            assert_eq!(expected, Solution::contains_duplicate(inputs));
        }
    }
}
