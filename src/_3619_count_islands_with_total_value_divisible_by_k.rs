//! Solution for https://leetcode.com/problems/count-islands-with-total-value-divisible-by-k
//! 3619. Count Islands With Total Value Divisible by K

impl Solution {
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        let columns = grid[0].len();

        let mut visited = vec![vec![false; columns]; rows];

        let mut count = 0;
        for x in 0..rows {
            for y in 0..columns {
                if let Some(sum) = Self::island_sum(x, y, k, &grid, &mut visited) {
                    if sum % k == 0 {
                        count += 1
                    }
                }
            }
        }

        return count;
    }

    fn island_sum(
        x: usize,
        y: usize,
        k: i32,
        grid: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> Option<i32> {
        let has_visited = visited
            .get(x)
            .and_then(|row| row.get(y))
            .map(|&has_visited| has_visited)?;
        let num = grid.get(x).and_then(|row| row.get(y)).map(|&num| num)?;

        if has_visited || num == 0 {
            return None;
        }

        visited[x][y] = true;
        let mut sum = num;
        if x > 0 {
            sum += Self::island_sum(x - 1, y, k, grid, visited).unwrap_or(0) % k;
        }
        if y > 0 {
            sum += Self::island_sum(x, y - 1, k, grid, visited).unwrap_or(0) % k;
        }
        sum += Self::island_sum(x, y + 1, k, grid, visited).unwrap_or(0) % k;
        sum += Self::island_sum(x + 1, y, k, grid, visited).unwrap_or(0) % k;

        return Some(sum);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![0,2,1,0,0],vec![0,5,0,0,5],vec![0,0,1,0,0],vec![0,1,4,7,0],vec![0,2,0,0,8]], 5, 2)]
    #[case(vec![vec![3,0,3,0],vec![0,3,0,3],vec![3,0,3,0]], 3, 6)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::count_islands(grid, k);
        assert_eq!(actual, expected);
    }
}
