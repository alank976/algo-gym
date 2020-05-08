// https://leetcode.com/problems/non-overlapping-intervals/
#[allow(dead_code)]
struct Solution;
//---------------------------------
#[allow(dead_code)]
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let state = (intervals.len() as i32 - 1 , intervals.first().map_or(0, |i| i[1]));
        let (overlapping_count, _) =
            intervals
                .into_iter()
                .skip(1)
                .fold(state, |mut state, interval| {
                    let (start, end) = (interval[0], interval[1]);
                    if start >= state.1 {
                        state.0 -= 1;
                    }
                    state.1 = end;
                    state
                });
        overlapping_count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        for (expected, input) in vec![
            (1, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            (2, vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            (0, vec![vec![1, 2], vec![2, 3]]),
        ] {
            assert_eq!(expected, Solution::erase_overlap_intervals(input));
        }
    }
}
