//! Solution for https://leetcode.com/problems/score-of-parentheses
//! 856. Score of Parentheses

use std::iter::once;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut level = 0;
        let mut result = 0;

        let previous = once(None).chain(s.chars().map(Some));
        let chars_with_prev = s.chars().zip(previous);

        for (char, prev) in chars_with_prev {
            match char {
                '(' => level += 1,
                ')' => {
                    level -= 1;
                    if let Some('(') = prev {
                        result += 1 << level;
                    }
                }
                _ => (),
            }
        }

        return result;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("()", 1)]
    #[case("(())", 2)]
    #[case("((()))", 4)]
    #[case("(()(())())", 8)]
    #[case("(()(()))", 6)]
    #[case("()()", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::score_of_parentheses(s);
        assert_eq!(actual, expected);
    }
}
