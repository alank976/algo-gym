//https://leetcode.com/problems/number-of-islands/
#[allow(dead_code)]
struct Solution {}
//-------------------------------------------
type Grid = Vec<Vec<char>>;

#[allow(dead_code)]
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        let mut counter = 0;
        let n_row = grid.len();
        let n_col = grid[0].len();
        for i in 0..n_row {
            for j in 0..n_col {
                if is_land(&grid, i, j) {
                    counter += 1;
                    flood(&mut grid, i, j);
                }
            }
        }
        counter
    }
}

fn flood(grid: &mut Grid, i: usize, j: usize) {
    let n_row = grid.len();
    let n_col = grid[0].len();
    if i < n_row && j < n_col && grid[i][j] == '1' {
        grid[i][j] = '0';
        flood(grid, i, j + 1);
        flood(grid, i + 1, j);
        if i > 0 {
            flood(grid, i - 1, j);
        }
        if j > 0 {
            flood(grid, i, j - 1);
        }
    }
}

fn is_land(grid: &Grid, i: usize, j: usize) -> bool {
    grid[i][j] == '1'
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
