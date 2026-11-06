//! Solution for https://leetcode.com/problems/twisted-mirror-path-count
//! 3665. Twisted Mirror Path Count

const MODULO: i32 = 1000000007;

enum Cell {
    Empty,
    Mirror,
}

struct Grid {
    grid: Vec<Vec<i32>>,
}

impl Grid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self { grid }
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        match self.grid.get(x)?.get(y)? {
            0 => Some(Cell::Empty),
            _ => Some(Cell::Mirror),
        }
    }

    fn get_next_right(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        return match self.get(x + 1, y)? {
            Cell::Empty => Some((x + 1, y)),
            Cell::Mirror => self.get_next_down(x + 1, y),
        };
    }

    fn get_next_down(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        return match self.get(x, y + 1)? {
            Cell::Empty => Some((x, y + 1)),
            Cell::Mirror => self.get_next_right(x, y + 1),
        };
    }
}

struct Path {
    grid: Vec<Vec<Option<i32>>>,
}

impl Path {
    fn new(n: usize, m: usize) -> Self {
        let mut path = Self {
            grid: vec![vec![None; m]; n],
        };

        path.grid[n - 1][m - 1] = Some(1);
        return path;
    }

    fn get_path_count(&self, x: usize, y: usize) -> Option<&i32> {
        return self.grid.get(x)?.get(y)?.as_ref();
    }

    fn update_path_count(&mut self, x: usize, y: usize, count: i32) {
        if let Some(row) = self.grid.get_mut(x) {
            if let Some(cell) = row.get_mut(y) {
                *cell = Some(count);
            }
        }
    }
}

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        let mut paths = Path::new(grid.len(), grid[0].len());
        let grid = Grid::new(grid);

        let results = Self::dfs(0, 0, &grid, &mut paths);
        return results;
    }

    fn dfs(x: usize, y: usize, grid: &Grid, paths: &mut Path) -> i32 {
        if let Some(paths_count) = paths.get_path_count(x, y) {
            return *paths_count;
        }

        let right = match grid.get_next_right(x, y) {
            None => 0,
            Some((x, y)) => Self::dfs(x, y, grid, paths),
        };

        let down = match grid.get_next_down(x, y) {
            None => 0,
            Some((x, y)) => Self::dfs(x, y, grid, paths),
        };

        let paths_count = (right + down) % MODULO;
        paths.update_path_count(x, y, paths_count);

        return paths_count;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,1,0],vec![0,0,1],vec![1,0,0]], 5)]
    #[case(vec![vec![0,0],vec![0,0]], 2)]
    #[case(vec![vec![0,1,1],vec![1,1,0]], 1)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::unique_paths(grid);
        assert_eq!(actual, expected);
    }
}
