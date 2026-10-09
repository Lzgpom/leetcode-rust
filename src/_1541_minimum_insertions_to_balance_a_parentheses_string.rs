//! Solution for https://leetcode.com/problems/minimum-insertions-to-balance-a-parentheses-string
//! 1541. Minimum Insertions to Balance a Parentheses String

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let mut iterator = s.chars().peekable();
        let mut level = 0;
        let mut result = 0;

        while let Some(char) = iterator.next() {
            match char {
                '(' => level += 2,
                _ => {
                    if iterator.next_if_eq(&')').is_none() {
                        result += 1;
                    }
                    match level {
                        0 => result += 1,
                        _ => level -= 2,
                    }
                }
            }
        }

        return result + level;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(()))", 1)]
    #[case("())", 0)]
    #[case(")))))))", 5)]
    #[case("))())(", 3)]
    #[case("()())))()", 3)]
    #[case(")))())()()())()((()((()((())))()((", 23)]
    #[case("))(()()))()))))))()())()(())()))))()())(()())))()(", 16)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_insertions(s);
        assert_eq!(actual, expected);
    }
}
