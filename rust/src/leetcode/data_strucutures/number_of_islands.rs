//https://leetcode.com/problems/number-of-islands/
#[allow(dead_code)]
struct Solution {}
//-------------------------------------------

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut scanner = Scanner::new(&grid);
        let mut counter = 0;
        for (i, row) in grid.iter().enumerate() {
            for j in 0..row.len() {
                if scanner.is_island(i, j) && !scanner.is_visited(i, j) {
                    Self::scan(&mut scanner, i, j);
                    counter += 1;
                }
            }
        }
        counter
    }

    fn scan(scanner: &mut Scanner, i: usize, j: usize) {
        if scanner.is_island(i, j) && !scanner.is_visited(i, j) {
            scanner.visit(i, j);
            for (adj_i, adj_j) in scanner.get_adjacent_unseen_islands(i, j) {
                Self::scan(scanner, adj_i, adj_j);
            }
        }
    }
}

struct Scanner<'a> {
    grid: &'a Vec<Vec<char>>,
    visited: Vec<Vec<bool>>,
    n_row: usize,
    n_col: usize,
}

impl<'a> Scanner<'a> {
    fn new(grid: &'a Vec<Vec<char>>) -> Self {
        let n_row = grid.len();
        let n_col = grid[0].len();
        Self {
            grid,
            visited: vec![vec![false; n_col]; n_row],
            n_row,
            n_col,
        }
    }

    fn is_island(&self, i: usize, j: usize) -> bool {
        i < self.n_row && j < self.n_col && self.grid[i][j] == '1'
    }

    fn visit(&mut self, i: usize, j: usize) {
        self.visited[i][j] = true;
    }

    fn is_visited(&self, i: usize, j: usize) -> bool {
        self.visited[i][j]
    }

    fn get_adjacent_unseen_islands(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut adj = vec![(i, j + 1), (i + 1, j)];
        if i > 0 {
            adj.push((i - 1, j));
        }
        if j > 0 {
            adj.push((i, j - 1));
        }
        adj.into_iter()
            .filter(|&(adj_i, adj_j)| self.is_island(adj_i, adj_j) && !self.visited[adj_i][adj_j])
            .collect()
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        for (expected, input) in vec![
            (
                1,
                vec![
                    "11110".chars().collect(),
                    "11010".chars().collect(),
                    "11000".chars().collect(),
                    "00000".chars().collect(),
                ],
            ),
            (
                3,
                vec![
                    "11000".chars().collect(),
                    "11000".chars().collect(),
                    "00100".chars().collect(),
                    "00011".chars().collect(),
                ],
            ),
            (0, vec![]),
        ] {
            assert_eq!(expected, Solution::num_islands(input));
        }
    }
}
