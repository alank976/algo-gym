// https://leetcode.com/problems/3sum/

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        // a+b+c=0 => b+c = -a
        if nums.len() < 3 {
            return vec![];
        }
        let mut nums = nums.clone();
        nums.sort_unstable();
        let mut results: Vec<Vec<i32>> = vec![];
        let mut prev_num: Option<i32> = None;
        for (i, &a) in nums.iter().enumerate() {
            let target = -a;
            if prev_num.map_or_else(|| false, |prev| prev == a) {
                // skip if same as previous one
                continue;
            }
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let sum = nums[left] + nums[right];
                if sum < target {
                    // move left in order to have greater sum
                    left += 1;
                } else if sum > target {
                    // move right to tune sum smaller
                    right -= 1;
                } else {
                    let (b, c) = (nums[left], nums[right]);
                    results.push(vec![a, b, c]);
                    while {
                        left += 1;
                        right -= 1;
                        left < right && b == nums[left] && c == nums[right]
                    } {}
                }
            }
            prev_num = Some(a);
        }
        results
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_it_works() {
        for (expected, inputs) in vec![
            (
                vec![vec![-1, 0, 1], vec![-1, -1, 2]],
                vec![-1, 0, 1, 2, -1, -4],
            ),
            (vec![vec![-6, 0, 6]], vec![-6, 0, 0, 6, 6]),
            (
                vec![vec![-6, 0, 6], vec![-6, 2, 4], vec![-1, 0, 1]],
                vec![-6, -1, 0, 1, 2, 4, 6],
            ),
            (vec![vec![-1, 0, 1], vec![0, 0, 0]], vec![-1, 0, 0, 0, 1]),
            (
                vec![
                    vec![-5, 1, 4],
                    vec![-4, 0, 4],
                    vec![-4, 1, 3],
                    vec![-2, -2, 4],
                    vec![-2, 1, 1],
                    vec![0, 0, 0],
                ],
                vec![-4, -2, 1, -5, -4, -4, 4, -2, 0, 4, 0, -2, 3, 1, -5, 0],
            ),
            (vec![vec![-2, 0, 2]], vec![-2, 0, 0, 2, 2]),
        ] {
            let expected_set: HashSet<Vec<i32>> = expected.into_iter().collect();
            let result_set: HashSet<Vec<i32>> = Solution::three_sum(inputs).into_iter().collect();
            assert_eq!(expected_set, result_set);
        }
    }
}
