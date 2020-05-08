// https://leetcode.com/problems/merge-intervals/

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::simplified(intervals)
    }

    fn merge_without_sort(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals
            .into_iter()
            .map(|vec| (vec[0], vec[1]))
            .collect::<Vec<(i32, i32)>>();
        intervals.sort();

        let (mut results, last_interval) =
            intervals
                .into_iter()
                .fold(
                    (vec![], None),
                    |(mut results, prev_interval), interval| match prev_interval {
                        None => (results, Some(interval)),
                        Some(prev) if prev.is_overlap(&interval) => {
                            (results, Some(interval.merge(prev)))
                        }
                        Some(prev) => {
                            results.push(prev);
                            (results, Some(interval))
                        }
                    },
                );
        if let Some(interval) = last_interval {
            results.push(interval);
        }
        results
            .into_iter()
            .map(|(start, end)| vec![start, end])
            .collect()
    }

    fn simplified(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() <= 1 {
            return intervals;
        }
        let mut interval_tuples: Vec<(i32, i32)> =
            intervals.into_iter().map(|vec| (vec[0], vec[1])).collect();
        interval_tuples.sort();
        let first: (i32, i32) = interval_tuples[0];
        interval_tuples
            .into_iter()
            .skip(1)
            .fold(vec![first], |mut merged_intervals, interval| {
                let (start, end) = interval;
                let (prev_start, prev_end) = merged_intervals.pop().unwrap();
                if start <= prev_end {
                    // is overlapped
                    merged_intervals.push((prev_start, end.max(prev_end)));
                } else {
                    merged_intervals.push((prev_start, prev_end));
                    merged_intervals.push(interval);
                }
                merged_intervals
            })
            .into_iter()
            .map(|(start, end)| vec![start, end])
            .collect()
    }
}
trait Interval<T> {
    fn is_overlap(&self, other: &T) -> bool;
    fn is_in(&self, x: i32) -> bool;
    fn merge(self, other: T) -> Self;
}

impl Interval<(i32, i32)> for (i32, i32) {
    fn is_overlap(&self, other: &(i32, i32)) -> bool {
        let (start, end) = *other;
        self.is_in(start) || self.is_in(end)
    }

    fn is_in(&self, x: i32) -> bool {
        self.0 <= x && x <= self.1
    }

    fn merge(self, other: Self) -> Self {
        (self.0.min(other.0), self.1.max(other.1))
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_it_works() {
        for (expected, inputs) in vec![
            (
                vec![vec![1, 10]],
                vec![vec![2, 3], vec![4, 5], vec![6, 7], vec![8, 9], vec![1, 10]],
            ),
            (vec![vec![1, 4], vec![5, 6]], vec![vec![1, 4], vec![5, 6]]),
            (
                vec![vec![1, 6], vec![8, 10], vec![15, 18]],
                vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]],
            ),
            (vec![vec![1, 5]], vec![vec![1, 4], vec![4, 5]]),
        ] {
            assert_eq!(expected, Solution::merge(inputs))
        }
    }
}
