//! Solution for https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses
//! 1614. Maximum Nesting Depth of the Parentheses

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;

        for char in s.bytes() {
            match char {
                b'(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth);
                }
                b')' => depth -= 1,
                _ => {}
            }
        }

        return max_depth;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(1+(2*3)+((8)/4))+1", 3)]
    #[case("(1)+((2))+(((3)))", 3)]
    #[case("()(())((()()))", 3)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::max_depth(s);
        assert_eq!(actual, expected);
    }
}
