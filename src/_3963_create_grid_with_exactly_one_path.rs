//! Solution for https://leetcode.com/problems/create-grid-with-exactly-one-path
//! 3963. Create Grid With Exactly One Path

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let m = m as usize;
        let n = n as usize;
        let mut grid = Vec::with_capacity(m as usize);
        grid.push(".".repeat(n));
        for _ in 1..m {
            let mut row = "#".repeat(n - 1);
            row.push('.');
            grid.push(row);
        }

        return grid;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(2, 3, vec!["...".into(),"##.".into()])]
    #[case(3, 3, vec!["...".into(),"##.".into(),"##.".into()])]
    #[case(1, 4, vec!["....".into()])]
    fn case(#[case] m: i32, #[case] n: i32, #[case] expected: Vec<String>) {
        let actual = Solution::create_grid(m, n);
        assert_eq!(actual, expected);
    }
}
