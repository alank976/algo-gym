// https://leetcode.com/problems/pacific-atlantic-water-flow/
#[allow(dead_code)]
struct Solution;
// ------------------------------------------------
#[allow(dead_code)]
impl Solution {
    pub fn pacific_atlantic(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix.first().map_or(0, |row| row.len());
        if m <= 1 || n <= 1 {
            // empty, singleton row/column => all elements can flow both
            let mut result = Vec::new();
            for i in 0..m {
                for j in 0..n {
                    result.push(vec![i as i32, j as i32]);
                }
            }
            result
        } else {
            let mut flow_explorer = FlowExplorer::new(&matrix);
            let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

            (0..n)
                .map(|x| (0, x))
                .chain((0..m).map(|x| (x, 0)))
                // coordinates of pacific shoreline
                .for_each(|c| flow_explorer.explore(c, &Ocean::Pacific, &offsets));

            (0..n)
                .map(|x| (m - 1, x))
                .chain((0..m).map(|x| (x, n - 1)))
                // coordinates of altantic shoreline
                .for_each(|c| flow_explorer.explore(c, &Ocean::Altantic, &offsets));

            (0..m)
                .flat_map(|i| (0..n).map(move |j| (i, j)))
                .filter(|&(i, j)| flow_explorer.visited[i][j].can_visit_both())
                .map(|(i, j)| vec![i as i32, j as i32])
                .collect()
        }
    }
}

enum Ocean {
    Pacific,
    Altantic,
}

type Coordinate = (usize, usize);
#[derive(Clone)]
struct Visited(Option<()>, Option<()>);

impl Visited {
    fn can_visit_both(&self) -> bool {
        self.0.and(self.1).is_some()
    }
    fn new() -> Self {
        Visited(None, None)
    }
}

struct FlowExplorer<'a> {
    matrix: &'a Vec<Vec<i32>>,
    m: usize,
    n: usize,
    visited: Vec<Vec<Visited>>,
}
impl<'a> FlowExplorer<'a> {
    fn new(matrix: &'a Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix.first().map_or(0, |row| row.len());
        FlowExplorer {
            matrix,
            m,
            n,
            visited: vec![vec![Visited::new(); n]; m],
        }
    }
    fn explore(
        &mut self,
        coordinate: Coordinate,
        from_ocean: &Ocean,
        neighbour_offsets: &Vec<(i32, i32)>,
    ) {
        let (i, j) = coordinate;
        match from_ocean {
            Ocean::Pacific => self.visited[i][j].0 = Some(()),
            Ocean::Altantic => self.visited[i][j].1 = Some(()),
        };
        let (m, n) = (self.m as i32, self.n as i32);
        for (offset_i, offset_j) in neighbour_offsets {
            let (ni, nj) = (i as i32 + offset_i, j as i32 + offset_j);
            if ni >= 0 && ni < m && nj >= 0 && nj < n {
                let nb = (ni as usize, nj as usize);
                let flowed = &self.visited[nb.0][nb.1];
                let visited = match from_ocean {
                    Ocean::Pacific => flowed.0,
                    Ocean::Altantic => flowed.1,
                };
                if visited.is_none() && self.matrix[i][j] <= self.matrix[nb.0][nb.1] {
                    self.explore(nb, from_ocean, neighbour_offsets)
                }
            }
        }
    }
}

#[cfg(test)]
mod test_data_struct {
    use super::*;

    #[test]
    fn test_pacific_atlantic() {
        for (expected, input) in vec![
            (Vec::new(), Vec::new()),
            (
                vec![
                    vec![0, 4],
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 2],
                    vec![3, 0],
                    vec![3, 1],
                    vec![4, 0],
                ],
                vec![
                    vec![1, 2, 2, 3, 5],
                    vec![3, 2, 3, 4, 4],
                    vec![2, 4, 5, 3, 1],
                    vec![6, 7, 1, 4, 5],
                    vec![5, 1, 1, 2, 4],
                ],
            ),
            (
                vec![
                    vec![0, 2],
                    vec![1, 0],
                    vec![1, 1],
                    vec![1, 2],
                    vec![2, 0],
                    vec![2, 1],
                    vec![2, 2],
                ],
                vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            ),
            (
                vec![],
                vec![
                    vec![1, 2, 2, 3, 5],
                    vec![3, 2, 3, 4, 4],
                    vec![2, 4, 5, 3, 1],
                    vec![6, 7, 1, 4, 5],
                    vec![5, 1, 1, 2, 4],
                ],
            ),
        ] {
            let result = Solution::pacific_atlantic(input);
            for e in expected {
                assert!(result.contains(&e));
            }
        }
    }
}
