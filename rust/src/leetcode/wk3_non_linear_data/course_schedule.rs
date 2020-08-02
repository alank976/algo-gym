// https://leetcode.com/problems/course-schedule/

#[allow(dead_code)]
struct Solution;
// --------------------------------
const COURSE: usize = 0;
const PREREQUISITE: usize = 1;

#[allow(dead_code)]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let prereqs_by_course = Self::prereqs_by_course(prerequisites, num_courses);
        let mut is_not_cyclic_by_course: Vec<bool> = vec![false; num_courses];
        let mut is_seen_by_course: Vec<bool> = vec![false; num_courses];

        !(0..num_courses).any(|course| {
            Self::is_cycle(
                &mut is_seen_by_course,
                course,
                &mut is_not_cyclic_by_course,
                &prereqs_by_course,
            )
        })
    }

    fn is_cycle(
        is_seen: &mut Vec<bool>,
        course: usize,
        is_not_cyclic: &mut Vec<bool>,
        prereqs_by_course: &Vec<Vec<usize>>,
    ) -> bool {
        if is_not_cyclic[course] {
            return false;
        }
        if is_seen[course] {
            return true;
        }
        is_seen[course] = true;
        let is_cycle = prereqs_by_course[course].iter().any(|&prereq_course| {
            Self::is_cycle(is_seen, prereq_course, is_not_cyclic, prereqs_by_course)
        });
        if !is_cycle {
            is_not_cyclic[course] = true;
        }
        is_cycle
    }

    fn prereqs_by_course(prerequisites: Vec<Vec<i32>>, num_courses: usize) -> Vec<Vec<usize>> {
        prerequisites
            .into_iter()
            .fold(vec![Vec::new(); num_courses], |mut vv, p| {
                let course = p[COURSE] as usize;
                let prereq = p[PREREQUISITE] as usize;
                vv[course].push(prereq);
                vv
            })
    }
}

#[cfg(test)]
mod test_non_linear_data {
    use super::*;

    #[test]
    fn test_can_finish() {
        for (expected, (num_courses, pre_req)) in vec![
            (true, (2, vec![vec![1, 0]])),
            (false, (2, vec![vec![1, 0], vec![0, 1]])),
            (true, (1, vec![])),
            (
                false,
                (4, vec![vec![0, 1], vec![1, 2], vec![1, 0], vec![2, 3]]),
            ),
            (false, (3, vec![vec![1, 0], vec![2, 0], vec![0, 1]])),
            (false, (3, vec![vec![0, 2], vec![1, 2], vec![2, 0]])),
        ] {
            assert_eq!(expected, Solution::can_finish(num_courses, pre_req))
        }
    }
}
