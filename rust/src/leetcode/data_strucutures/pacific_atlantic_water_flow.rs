// https://leetcode.com/problems/pacific-atlantic-water-flow/
#[allow(dead_code)]
struct Solution;
// ------------------------------------------------
use std::collections::HashSet;
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

            flow_explorer
                .altantic_water_flowed
                .intersection(&flow_explorer.pacific_water_flowed)
                .map(|&(x, y)| vec![x as i32, y as i32])
                .collect()
        }
    }
}

enum Ocean {
    Pacific,
    Altantic,
}

type Coordinate = (usize, usize);

struct FlowExplorer<'a> {
    matrix: &'a Vec<Vec<i32>>,
    m: usize,
    n: usize,
    pacific_water_flowed: HashSet<Coordinate>,
    altantic_water_flowed: HashSet<Coordinate>,
    // neighbour_offsets: Vec<(i32, i32)>,
}
impl<'a> FlowExplorer<'a> {
    fn new(matrix: &'a Vec<Vec<i32>>) -> Self {
        let m = matrix.len();
        let n = matrix.first().map_or(0, |row| row.len());
        FlowExplorer {
            matrix,
            m,
            n,
            pacific_water_flowed: HashSet::new(),
            altantic_water_flowed: HashSet::new(),
            // neighbour_offsets: vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        }
    }
    fn explore(
        &mut self,
        coordinate: Coordinate,
        from_ocean: &Ocean,
        neighbour_offsets: &Vec<(i32, i32)>,
    ) {
        match from_ocean {
            Ocean::Pacific => &mut self.pacific_water_flowed,
            Ocean::Altantic => &mut self.altantic_water_flowed,
        }
        .insert(coordinate);
        let (i, j) = coordinate;
        let (m, n) = (self.m as i32, self.n as i32);
        // let offsets = &self.neighbour_offsets;
        for (offset_i, offset_j) in neighbour_offsets {
            let (ni, nj) = (i as i32 + offset_i, j as i32 + offset_j);
            if ni >= 0 && ni < m && nj >= 0 && nj < n {
                let nb = (ni as usize, nj as usize);
                let flowed = match from_ocean {
                    Ocean::Pacific => &self.pacific_water_flowed,
                    Ocean::Altantic => &self.altantic_water_flowed,
                };
                if !flowed.contains(&nb) && self.matrix[i][j] <= self.matrix[nb.0][nb.1] {
                    self.explore(nb, from_ocean, neighbour_offsets)
                }
            }
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
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
            println!("result = {:?}", result);
            for e in expected {
                println!("expected to have  = {:?}", e);
                assert!(result.contains(&e));
            }
        }
    }

    #[test]
    fn test_aaa() {
        let mut a = 0;
        a |= 1;
        a |= 2;
        println!("a={}", a);
    }
}
