/*
https://leetcode.com/problems/two-sum/
*/

use std::collections::HashMap;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut index_by_value: HashMap<i32, usize> = HashMap::new();
        nums.iter()
            .enumerate()
            .find_map(|(i, &x)| {
                let complement = target - x;
                let found_pair = index_by_value
                    .get(&complement)
                    .map(|&seen_complement_index| vec![seen_complement_index as i32, i as i32]);
                index_by_value.insert(x, i);
                found_pair
            })
            .unwrap_or(vec![])
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_it_works() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }
}
