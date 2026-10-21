//! Solution for https://leetcode.com/problems/minimum-operations-to-sort-a-string
//! 3863. Minimum Operations to Sort a String

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let chars = s.as_bytes();
        let len = chars.len();

        let first = chars[0];
        let last = chars[len - 1];

        let mut sorted = true;
        let mut min = chars[0];
        let mut max = chars[0];

        let mut mid_min = b'z';
        let mut mid_max = b'a';
        for i in 1..len {
            let char = chars[i];
            if sorted && char < chars[i - 1] {
                sorted = false;
            }

            min = min.min(char);
            max = max.max(char);

            if i < len - 1 {
                mid_min = mid_min.min(char);
                mid_max = mid_max.max(char)
            }
        }

        if sorted {
            return 0;
        }

        if len == 2 {
            return -1;
        }

        if first == min || last == max {
            return 1;
        }

        if first == max && last == min && min != mid_min && max != mid_max {
            return 3;
        }

        return 2;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("dog", 1)]
    #[case("card", 2)]
    #[case("gf", -1)]
    #[case("cab", 2)]
    #[case("edc", 3)]
    #[case("edbc", 2)]
    #[case("jgg", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_operations(s);
        assert_eq!(actual, expected);
    }
}
