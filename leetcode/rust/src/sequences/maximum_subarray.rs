use std::cmp::max;
use std::ops::Range;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // O(n*k) + other complexity?
        // Self::negative_driven(&nums)
        // O(nlogn)
        // Self::divide_and_conquer(&nums, 0..nums.len())
        // O(n)
        Self::kadane(&nums)
    }

    fn kadane(nums: &Vec<i32>) -> i32 {
        let mut best_sum = i32::min_value();
        let mut current_sum = 0;
        for &x in nums {
            current_sum = max(x, current_sum + x);
            best_sum = max(best_sum, current_sum);
        }
        best_sum
    }

    fn divide_and_conquer(nums: &Vec<i32>, range: Range<usize>) -> i32 {
        if range.end - range.start <= 1 {
            return nums[range.start];
        }
        let mid_point = (range.start + range.end) / 2;
        let maxs = vec![
            Self::divide_and_conquer(&nums, range.start..mid_point),
            Self::divide_and_conquer(&nums, mid_point..range.end),
            Self::max_cross_mid_point(&nums, range, mid_point),
        ];
        *maxs.iter().max().unwrap()
    }

    fn max_cross_mid_point(nums: &Vec<i32>, range: Range<usize>, mid: usize) -> i32 {
        let mut left_max = i32::min_value();
        let mut sum = 0;
        for i in (range.start..mid).rev() {
            sum += nums[i];
            if sum > left_max {
                left_max = sum;
            }
        }
        let mut right_max = i32::min_value();
        sum = 0;
        for i in mid..range.end {
            sum += nums[i];
            if sum > right_max {
                right_max = sum;
            }
        }
        left_max + right_max
    }

    fn negative_driven(nums: &Vec<i32>) -> i32 {
        let negatives = find_negatives(&nums);
        if negatives.is_empty() {
            return nums.iter().sum();
        }
        if nums.iter().all(|&num| num < 0) {
            return *nums.iter().max().unwrap();
        }
        let mut left_start = 0;
        let mut max = 0;
        for i in 0..negatives.len() {
            let neg = &negatives[i];
            let left_part = left_start..neg.start;
            let right_part = &negatives
                .get(i + 1)
                .map(|next_neg| neg.end..next_neg.start)
                .unwrap_or(neg.end..nums.len());
            // if either sum of left or that of right is greater than the negative one, this become next left
            let left_sum = sum(left_part, &nums);
            let neg_sum: i32 = sum(neg.clone(), &nums);
            let right_sum = sum(right_part.clone(), &nums);
            if left_sum + neg_sum < 0 || right_sum + neg_sum < 0 {
                // No need to consider left part in the subsequent part
                // However, left itself might be the max so memorize it
                if left_sum > max {
                    max = left_sum;
                }
                // update next left start index
                if right_sum > left_sum + neg_sum + right_sum {
                    left_start = neg.end;
                }
            } else if right_part.end == nums.len() {
                let this_sum = left_sum + neg_sum + right_sum;
                if this_sum > max {
                    max = this_sum;
                }
            }
        }
        let final_right_sum = sum(left_start..nums.len(), &nums);
        if final_right_sum > max {
            max = final_right_sum;
        }
        max
    }
}

fn sum(range: Range<usize>, nums: &Vec<i32>) -> i32 {
    range.map(|i| nums[i]).sum()
}

fn find_negatives(nums: &Vec<i32>) -> Vec<Range<usize>> {
    let mut negatives = vec![];
    let mut last_neg_i = None;
    for (i, &num) in nums.iter().enumerate() {
        match last_neg_i {
            Some(lni) => {
                if num >= 0 {
                    negatives.push(lni..i);
                    last_neg_i = None;
                }
            }
            None => {
                if num < 0 {
                    last_neg_i = Some(i);
                };
            }
        }
    }
    if let Some(lni) = last_neg_i {
        negatives.push(lni..nums.len());
    }
    negatives
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn it_works() {
        for (expected, input) in vec![
            (-1, vec![-1]),
            (-2, vec![-5, -2]),
            (-1, vec![-2, -1]),
            (
                33,
                vec![-9, -2, 1, 8, 7, -6, 4, 9, -9, -5, 0, 5, -2, 5, 9, 7],
            ),
            (6, vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            (51, vec![1, 2, -1, -1, 50]),
        ] {
            assert_eq!(expected, Solution::max_sub_array(input));
        }
    }
}
