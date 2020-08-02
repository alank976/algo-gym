// https://leetcode.com/problems/maximum-product-subarray/

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
// ----------------
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        Self::divide_and_conquer(0, nums.len() - 1, &nums)
    }

    fn divide_and_conquer(start: usize, end: usize, nums: &Vec<i32>) -> i32 {
        if start == end {
            return nums[start];
        }
        let mid = (start + end + 1) / 2;
        Self::divide_and_conquer(start, mid - 1, nums)
            .max(Self::divide_and_conquer(mid, end, nums))
            .max(Self::cross_mid(start, mid, end, nums))
    }

    fn cross_mid(start: usize, mid: usize, end: usize, nums: &Vec<i32>) -> i32 {
        let mut product = nums[mid - 1];
        let mut left_max = (product, product);
        for i in (start..mid - 1).rev() {
            product *= nums[i];
            if product < 0 && product < left_max.0 {
                left_max.0 = product;
            }
            if product >= 0 && product > left_max.1 {
                left_max.1 = product;
            }
        }
        let mut product = nums[mid];
        let mut right_max = (product, product);
        for i in mid + 1..=end {
            product *= nums[i];
            if product < 0 && product < right_max.0 {
                right_max.0 = product;
            }
            if product >= 0 && product > right_max.1 {
                right_max.1 = product;
            }
        }
        (left_max.0 * right_max.0).max(left_max.1 * right_max.1)
    }
}

#[cfg(test)]
mod test_sequences {
    use super::*;

    #[test]
    fn test_max_products() {
        for (expected, inputs) in vec![
            (6, vec![2, 3, -2, 4]),
            (0, vec![-2, 0, -1]),
            (36, vec![-2, 3, -6, 1]),
        ] {
            assert_eq!(expected, Solution::max_product(inputs));
        }
    }
}
