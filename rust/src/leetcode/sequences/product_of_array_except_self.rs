// https://leetcode.com/problems/product-of-array-except-self/

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        for _ in &nums {
            result.push(1);
        }
        let n = nums.len();
        let mut left = 1; // product of all numbers on i's left
        let mut right = 1; // similarly but n-1-i

        for (i, left_num) in nums.iter().enumerate() {
            if let Some(r) = result.get_mut(i) {
                *r *= left;
            }
            left *= left_num;
            let right_num = nums[n - 1 - i];
            if let Some(r) = result.get_mut(n - 1 - i) {
                *r *= right;
            }
            right *= right_num;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        for (expected, input) in vec![(vec![24, 12, 8, 6], vec![1, 2, 3, 4])] {
            assert_eq!(expected, Solution::product_except_self(input));
        }
    }
}
