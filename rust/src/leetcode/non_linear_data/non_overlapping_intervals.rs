// https://leetcode.com/problems/non-overlapping-intervals/
#[allow(dead_code)]
struct Solution;
//---------------------------------
#[allow(dead_code)]
impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        intervals.sort_unstable_by(|a, b| a[1].cmp(&b[1]));
        let length = intervals.len() as i32;
        let state = (1, intervals.first().map_or(0, |x| x[1]));
        let (non_overlapping_count, _) =
            intervals
                .into_iter()
                .skip(1)
                .fold(state, |(mut count, mut prev_end), interval| {
                    let (start, end) = (interval[0], interval[1]);
                    if start >= prev_end {
                        // overlapped
                        count += 1;
                        // only update end when overlapped, since it will be "erased"
                        prev_end = end;
                    }
                    (count, prev_end)
                });
        length - non_overlapping_count
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        for (expected, input) in vec![
            (0, vec![]),
            (
                0,
                vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![-100, -2],
                    vec![5, 7],
                ],
            ),
            (
                2,
                vec![vec![1, 100], vec![11, 22], vec![1, 11], vec![2, 12]],
            ),
            (1, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
            (2, vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
            (0, vec![vec![1, 2], vec![2, 3]]),
        ] {
            assert_eq!(expected, Solution::erase_overlap_intervals(input));
        }
    }
}
