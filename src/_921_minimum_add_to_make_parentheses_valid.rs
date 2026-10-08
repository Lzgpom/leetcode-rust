//! Solution for https://leetcode.com/problems/minimum-add-to-make-parentheses-valid
//! 921. Minimum Add to Make Parentheses Valid

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut level = 0;
        let mut result = 0;

        for char in s.chars() {
            let diff = match char {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

            level += diff;
            if level < 0 {
                level = 0;
                result += 1;
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
    #[case("())", 1)]
    #[case("(((", 3)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_add_to_make_valid(s);
        assert_eq!(actual, expected);
    }
}
