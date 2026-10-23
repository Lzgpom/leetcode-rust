//! Solution for https://leetcode.com/problems/maximum-nesting-depth-of-two-valid-parentheses-strings
//! 1111. Maximum Nesting Depth of Two Valid Parentheses Strings

impl Solution {
    pub fn max_depth_after_split(seq: String) -> Vec<i32> {
        let mut stack = Vec::new();
        let mut result = Vec::with_capacity(seq.len());

        let mut a = 0;
        let mut b = 0;

        for char in seq.bytes() {
            match char {
                b'(' => {
                    if b > a {
                        a += 1;
                        stack.push(0);
                        result.push(0);
                    } else {
                        b += 1;
                        stack.push(1);
                        result.push(1);
                    }
                }
                _ => {
                    if let Some(k) = stack.pop() {
                        if k == 0 {
                            a -= 1;
                        } else {
                            b -= 1;
                        }

                        result.push(k);
                    }
                }
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
    #[case("(()())", vec![0,1,1,1,1,0])]
    #[case("()(())()", vec![0,0,0,1,1,0,1,1])]
    fn case(#[case] seq: String, #[case] expected: Vec<i32>) {
        let actual = Solution::max_depth_after_split(seq);
        assert_eq!(actual, expected);
    }
}
