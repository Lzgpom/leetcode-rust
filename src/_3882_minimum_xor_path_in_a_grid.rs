//! Solution for https://leetcode.com/problems/minimum-xor-path-in-a-grid
//! 3882. Minimum XOR Path in a Grid

use std::collections::VecDeque;

struct Position {
    x: usize,
    y: usize,
    value: i32,
}

impl Position {
    fn is_at_end(&self, m: usize, n: usize) -> bool {
        return self.x == m - 1 && self.y == n - 1;
    }

    fn can_go_down(&self, m: usize) -> bool {
        return self.x < m - 1;
    }

    fn can_go_right(&self, n: usize) -> bool {
        return self.y < n - 1;
    }
}

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut min_path = i32::MAX;
        let mut queue = VecDeque::new();
        queue.push_front(Position {
            x: 0,
            y: 0,
            value: grid[0][0],
        });

        while let Some(position) = queue.pop_back() {
            if position.is_at_end(m, n) {
                min_path = min_path.min(position.value);
                continue;
            }

            if position.can_go_down(m) {
                let x = position.x + 1;
                let y = position.y;
                queue.push_front(Position {
                    x,
                    y,
                    value: position.value ^ grid[x][y],
                });
            }

            if position.can_go_right(n) {
                let x = position.x;
                let y = position.y + 1;
                queue.push_front(Position {
                    x,
                    y,
                    value: position.value ^ grid[x][y],
                });
            }
        }

        return min_path;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,2],vec![3,4]], 6)]
    #[case(vec![vec![6,7],vec![5,8]], 9)]
    #[case(vec![vec![2,7,5]], 0)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::min_cost(grid);
        assert_eq!(actual, expected);
    }
}
