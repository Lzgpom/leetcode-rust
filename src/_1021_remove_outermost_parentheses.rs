//! Solution for https://leetcode.com/problems/remove-outermost-parentheses
//! 1021. Remove Outermost Parentheses

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut result = String::new();
        let mut level: u8 = 0;

        for char in s.chars() {
            match char {
                '(' => {
                    if level != 0 {
                        result.push(char);
                    }
                    level += 1;
                }
                ')' => {
                    level -= 1;
                    if level != 0 {
                        result.push(char);
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
    #[case("(()())(())", "()()()")]
    #[case("(()())(())(()(()))", "()()()()(())")]
    #[case("()()", "")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::remove_outer_parentheses(s);
        assert_eq!(actual, expected);
    }
}
