use core::ops::Range;

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let first_num = nums[0];
        if nums.len() == 1 || *nums.last().unwrap() > first_num {
            first_num
        } else {
            find(1..nums.len(), &first_num, &nums)
        }
    }
}

fn find(rng: Range<usize>, first_num: &i32, nums: &Vec<i32>) -> i32 {
    let mid = (rng.start + rng.end) / 2;
    if rng.len() == 1 {
        return nums[rng.start];
    }
    match nums[mid] {
        n if mid < nums.len() -1 && n > nums[mid + 1] => nums[mid + 1],
        n if nums[mid - 1] > n => n,
        n if n > *first_num => find(mid + 1..rng.end, first_num, nums),
        _ => find(rng.start..mid, first_num, nums),
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn it_works() {
        for (expected, input) in vec![
            (1, vec![3, 4, 5, 1, 2]),
            (0, vec![4, 5, 6, 7, 0, 1, 2]),
            (0, vec![1, 2, 3, 0]),
            (1, vec![4, 1, 2, 3]),
            (1, vec![1]),
            (1, vec![1, 2]),
            (1, vec![2, 1]),
            (1, vec![3, 1, 2]),
        ] {
            assert_eq!(expected, Solution::find_min(input));
        }
    }
}
