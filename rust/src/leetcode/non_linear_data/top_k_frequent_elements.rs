// https://leetcode.com/problems/top-k-frequent-elements/

#[allow(dead_code)]
struct Solution;

// ----------------------------------------------------------
use rand::Rng;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq)]
struct Element {
    num: i32,
    frequency: i32,
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let count_by_num = nums.into_iter().fold(HashMap::new(), |mut acc, num| {
            acc.entry(num).and_modify(|v| *v += 1).or_insert(1);
            acc
        });
        Self::heap_approach(count_by_num, k as usize)
        // let count_by_num_tuples: Vec<(i32, i32)> = count_by_num.into_iter().collect();
        // Self::quick_select_approach(count_by_num_tuples, k as usize)
        // Self::just_sort_approach(count_by_num_tuples, k as usize)
    }

    fn heap_approach(count_by_num: HashMap<i32, i32>, k: usize) -> Vec<i32> {
        let mut heap = BinaryHeap::new();
        for (num, frequency) in count_by_num {
            let element = Element { num, frequency };
            heap.push(element);
            if heap.len() > k {
                heap.pop();
            }
        }
        heap.into_iter().map(|x| x.num).collect()
    }

    fn just_sort_approach(mut count_by_num_tuples: Vec<(i32, i32)>, k: usize) -> Vec<i32> {
        count_by_num_tuples.sort_by(|a, b| b.1.cmp(&a.1));
        count_by_num_tuples
            .into_iter()
            .take(k)
            .map(|(key, _)| key)
            .collect()
    }

    fn quick_select_approach(mut count_by_num_tuples: Vec<(i32, i32)>, k: usize) -> Vec<i32> {
        let stop_index = count_by_num_tuples.len() - k;
        let right = count_by_num_tuples.len() - 1;
        Self::hoare_partition(&mut count_by_num_tuples, 0, right, stop_index);
        count_by_num_tuples
            .into_iter()
            .skip(stop_index)
            .map(|(num, _)| num)
            .collect()
    }

    fn hoare_partition(
        num_and_freqs: &mut Vec<(i32, i32)>,
        left: usize,
        right: usize,
        stop_index: usize,
    ) {
        let mut rng = rand::thread_rng();
        let pivot_index: usize = rng.gen_range(left, right + 1);
        let pivot_freq = num_and_freqs[pivot_index].1;
        num_and_freqs.swap(pivot_index, right);

        let mut store_index = left;
        for i in left..=right {
            if num_and_freqs[i].1 < pivot_freq {
                num_and_freqs.swap(i, store_index);
                store_index += 1;
            }
        }
        num_and_freqs.swap(store_index, right);

        match store_index {
            si if si > stop_index => Self::hoare_partition(num_and_freqs, left, si - 1, stop_index),
            si if si < stop_index => {
                Self::hoare_partition(num_and_freqs, si + 1, right, stop_index)
            }
            _ => (),
        };
    }
}

#[cfg(test)]
mod test_super {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_it_works() {
        for (expected, (nums, k)) in vec![
            (vec![1, 2], (vec![1, 1, 1, 2, 2, 3], 2)),
            (vec![1], (vec![1], 1)),
            (vec![1, 2], (vec![1, 1, 1, 2, 2, 33333333], 2)),
        ] {
            let unorder_expected: HashSet<i32> = expected.into_iter().collect();
            let unorder_result: HashSet<i32> =
                Solution::top_k_frequent(nums, k).into_iter().collect();
            assert_eq!(unorder_expected, unorder_result);
        }
    }
}
