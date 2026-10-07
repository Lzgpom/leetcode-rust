//! Solution for https://leetcode.com/problems/generate-parentheses
//! 22. Generate Parentheses

use std::vec;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::generate_parenthesis_dfs("".into(), 0, 0, n)
    }

    fn generate_parenthesis_dfs(current: String, start: i32, end: i32, n: i32) -> Vec<String> {
        if start == end && start == n {
            return vec![current];
        }

        let mut strings = Vec::<String>::new();
        if start <= n {
            strings.append(&mut Self::generate_parenthesis_dfs(
                current.clone() + "(",
                start + 1,
                end,
                n,
            ));
        }

        if end <= n && end < start {
            strings.append(&mut Solution::generate_parenthesis_dfs(
                current.clone() + ")",
                start,
                end + 1,
                n,
            ));
        }

        return strings;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec!["((()))".into(),"(()())".into(),"(())()".into(),"()(())".into(),"()()()".into()])]
    #[case(1, vec!["()".into()])]
    fn case(#[case] n: i32, #[case] expected: Vec<String>) {
        let actual = Solution::generate_parenthesis(n);
        assert_eq!(actual, expected);
    }
}
