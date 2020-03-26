#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        while left != right {
            let length = if height[left] > height[right] {
                height[right]
            } else {
                height[left]
            };
            let area = area(length, right - left);
            if area > max_area {
                max_area = area;
            }
            if height[left] > height[right] {
                right -= 1;
            } else {
                left += 1;
            };
        }
        max_area
    }
}

fn area(length: i32, width: usize) -> i32 {
    length * width as i32
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn it_works() {
        for (expected, input) in vec![(49, vec![1, 8, 6, 2, 5, 4, 8, 3, 7])] {
            assert_eq!(expected, Solution::max_area(input));
        }
    }
}
