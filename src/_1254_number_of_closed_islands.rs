//! Solution for https://leetcode.com/problems/number-of-closed-islands
//! 1254. Number of Closed Islands

use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

enum Type {
    Land,
    Water,
    Outside,
}

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = HashSet::<(i32, i32)>::new();
        let mut islands = 0;
        for x in 1..grid.len() - 1 {
            for y in 1..grid[x].len() - 1 {
                let x = x as i32;
                let y = y as i32;

                if !seen.contains(&(x, y)) {
                    if Self::is_island(x, y, &grid, &mut seen) {
                        islands += 1;
                    }
                }
            }
        }

        return islands;
    }

    fn is_island(x: i32, y: i32, grid: &Vec<Vec<i32>>, seen: &mut HashSet<(i32, i32)>) -> bool {
        match Self::cell_type(x, y, grid) {
            Type::Outside => false,
            Type::Water => false,
            Type::Land => Self::has_shore(x, y, grid, seen),
        }
    }

    fn has_shore(x: i32, y: i32, grid: &Vec<Vec<i32>>, seen: &mut HashSet<(i32, i32)>) -> bool {
        match Self::cell_type(x, y, grid) {
            Type::Outside => false,
            Type::Water => true,
            Type::Land => {
                if seen.contains(&(x, y)) {
                    return true;
                }
                seen.insert((x, y));

                let mut has_shore = true;
                for (h, v) in DIRECTIONS {
                    let new_x = x + h;
                    let new_y = y + v;
                    if !Self::has_shore(new_x, new_y, grid, seen) {
                        has_shore = false;
                    }
                }

                return has_shore;
            }
        }
    }

    fn cell_type(x: i32, y: i32, grid: &Vec<Vec<i32>>) -> Type {
        if x < 0 || y < 0 {
            return Type::Outside;
        }

        let x = x as usize;
        let y = y as usize;

        grid.get(x)
            .and_then(|row| row.get(y))
            .map(|cell| match cell {
                1 => Type::Water,
                _ => Type::Land,
            })
            .unwrap_or(Type::Outside)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,1,1,1,1,1,1,0],vec![1,0,0,0,0,1,1,0],vec![1,0,1,0,1,1,1,0],vec![1,0,0,0,0,1,0,1],vec![1,1,1,1,1,1,1,0]], 2)]
    #[case(vec![vec![0,0,1,0,0],vec![0,1,0,1,0],vec![0,1,1,1,0]], 1)]
    #[case(vec![vec![1,1,1,1,1,1,1],vec![1,0,0,0,0,0,1],vec![1,0,1,1,1,0,1],vec![1,0,1,0,1,0,1],vec![1,0,1,1,1,0,1],vec![1,0,0,0,0,0,1],vec![1,1,1,1,1,1,1]], 2)]
    #[case(vec![vec![0,0,1,1,0,1,0,0,1,0],vec![1,1,0,1,1,0,1,1,1,0],vec![1,0,1,1,1,0,0,1,1,0],vec![0,1,1,0,0,0,0,1,0,1],vec![0,0,0,0,0,0,1,1,1,0],vec![0,1,0,1,0,1,0,1,1,1],vec![1,0,1,0,1,1,0,0,0,1],vec![1,1,1,1,1,1,0,0,0,0],vec![1,1,1,0,0,1,0,1,0,1],vec![1,1,1,0,1,1,0,1,1,0]], 5)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::closed_island(grid);
        assert_eq!(actual, expected);
    }
}
