//! Solution for https://leetcode.com/problems/minimum-xor-path-in-a-grid
//! 3882. Minimum XOR Path in a Grid

const MAX_VALUE: usize = 1024;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp = vec![vec![vec![false; MAX_VALUE]; n]; m];
        dp[0][0][grid[0][0] as usize] = true;

        for x in 0..m {
            for y in 0..n {
                for (xor_value, _) in dp[x][y]
                    .clone()
                    .iter()
                    .enumerate()
                    .filter(|(_, flag)| **flag)
                {
                    if x < m - 1 {
                        let value = grid[x + 1][y] as usize;
                        dp[x + 1][y][xor_value ^ value] = true;
                    }
                    if y < n - 1 {
                        let value = grid[x][y + 1] as usize;
                        dp[x][y + 1][xor_value ^ value] = true;
                    }
                }
            }
        }

        return dp[m - 1][n - 1]
            .iter()
            .enumerate()
            .filter(|(_, flag)| **flag)
            .map(|(n, _)| n as i32)
            .min()
            .unwrap();
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
