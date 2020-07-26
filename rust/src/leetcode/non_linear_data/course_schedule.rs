// https://leetcode.com/problems/course-schedule/

// https://support.leetcode.com/hc/en-us/requests/37882

#[allow(dead_code)]
struct Solution;
// --------------------------------
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let prereqs_by_subjects = Self::group_by_subject(prerequisites);
        let mut subjects_without_cycle: HashSet<i32> = HashSet::new();

        !prereqs_by_subjects.keys().any(|&subj| {
            Self::is_cycle(
                subj,
                subj,
                true,
                &mut subjects_without_cycle,
                &prereqs_by_subjects,
            )
        })
    }

    fn is_cycle(
        start: i32,
        subj: i32,
        is_start: bool,
        subjects_without_cycle: &mut HashSet<i32>,
        prereqs_by_subjects: &HashMap<i32, Vec<i32>>,
    ) -> bool {
        if !is_start && start == subj {
            return true;
        }
        if subjects_without_cycle.contains(&subj) {
            return false;
        }
        let is_cycle = prereqs_by_subjects.get(&subj).map_or(false, |prereqs| {
            prereqs.iter().fold(false, |is_prev_cycle, &p| {
                is_prev_cycle
                    || Self::is_cycle(start, p, false, subjects_without_cycle, prereqs_by_subjects)
            })
        });
        if !is_cycle {
            subjects_without_cycle.insert(subj);
        }
        is_cycle
    }

    fn group_by_subject(prerequisites: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        prerequisites.into_iter().fold(HashMap::new(), |mut m, p| {
            let subj = p[0];
            let prereq = p[1]; // given it's a pair
            m.entry(subj)
                .and_modify(|e| e.push(prereq))
                .or_insert(vec![prereq]);
            m
        })
    }
}

#[cfg(test)]
mod test_non_linear_data {
    use super::*;

    #[test]
    fn test_can_finish() {
        for (expected, (num_courses, pre_req)) in vec![
            // (true, (2, vec![vec![1, 0]])),
            // (false, (2, vec![vec![1, 0], vec![0, 1]])),
            // (true, (1, vec![])),
            // (
            //     false,
            //     (1, vec![vec![0, 1], vec![1, 2], vec![1, 0], vec![2, 3]]),
            // ),
            (false, (3, vec![vec![1, 0], vec![2, 0], vec![0, 1]])),
            // (false, (3, vec![vec![0, 2], vec![1, 2], vec![2, 0]])),
        ] {
            assert_eq!(expected, Solution::can_finish(num_courses, pre_req))
        }
    }
}
