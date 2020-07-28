// https://leetcode.com/problems/search-in-rotated-sorted-array/

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// -----
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        if target == nums[0] {
            return 0;
        }
        Self::search_recursively((0, nums.len() - 1), target, &nums)
    }

    fn search_recursively(range: (usize, usize), target: i32, nums: &Vec<i32>) -> i32 {
        let pivot = nums[0];
        let (start, end) = range;
        if start == end && nums[start] != target {
            return -1;
        }
        let mid = (start + end + 1) / 2;
        let left = (start, (mid - 1).max(start));
        let right = ((mid + 1).min(end), end);
        match nums[mid] {
            m if m == target => mid as i32,
            m if m > pivot => {
                // value: __1__ p __2__ m __3__
                // pos: p __2__ m __1/3__
                if pivot < target && target < m {
                    Self::search_recursively(left, target, nums)
                } else {
                    Self::search_recursively(right, target, nums)
                }
            }
            m if m < pivot => {
                // value: __1__ m __2__ p __3__
                // pos: p __1/3__ m __2__
                if target > m && target < pivot {
                    Self::search_recursively(right, target, nums)
                } else {
                    Self::search_recursively(left, target, nums)
                }
            }
            _ => -1,
        }
    }
}

#[cfg(test)]
mod test_sequences {
    use super::*;

    #[test]
    fn test_search() {
        for (expected, (nums, target)) in vec![
            // (4, (vec![4, 5, 6, 7, 0, 1, 2], 0)),
            // (-1, (vec![4, 5, 6, 7, 0, 1, 2], 3)),
            // (2, (vec![4, 5, 6, 0, 1, 2, 3], 6)),
            // (2, (vec![4, 5, 0, 1, 2, 3, 4], 0)),
            // (6, (vec![4, 5, 6, 7, 0, 1, 2], 2)),
            // (-1, (vec![], 5)),
            (-1, (vec![1, 3], 0)),
        ] {
            assert_eq!(expected, Solution::search(nums, target))
        }
    }
}
