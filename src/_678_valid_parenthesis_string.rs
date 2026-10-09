//! Solution for https://leetcode.com/problems/valid-parenthesis-string
//! 678. Valid Parenthesis String

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut opens = Vec::new();
        let mut others = Vec::new();

        for (i, char) in s.chars().enumerate() {
            match char {
                '(' => opens.push(i),
                ')' => {
                    if opens.pop().is_none() {
                        if others.pop().is_none() {
                            return false;
                        }
                    }
                }
                _ => others.push(i),
            }
        }

        while let Some(open_index) = opens.pop() {
            if let Some(other_index) = others.pop() {
                if other_index < open_index {
                    return false;
                }
            } else {
                return false;
            }
        }

        return true;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("()", true)]
    #[case("(*)", true)]
    #[case("(*))", true)]
    #[case("(", false)]
    #[case("())(())", false)]
    #[case(
        "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())",
        false
    )]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::check_valid_string(s);
        assert_eq!(actual, expected);
    }
}
