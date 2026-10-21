//! Solution for https://leetcode.com/problems/minimum-operations-to-sort-a-string
//! 3863. Minimum Operations to Sort a String

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let chars = s.as_bytes();
        let len = chars.len();

        if len <= 1 {
            return 0;
        } else if len == 2 {
            if chars.is_sorted() {
                return 0;
            } else {
                return -1;
            }
        }

        let mut mid_sorted = true;
        let first = chars[0];
        let last = chars[len - 1];
        let mut prev = chars[1];
        let mut mid_min = chars[1];
        let mut mid_max = chars[1];

        for &char in &chars[1..len - 1] {
            if mid_sorted && prev > char {
                mid_sorted = false;
            }

            prev = char;
            mid_min = mid_min.min(char);
            mid_max = mid_max.max(char);
        }

        if first <= mid_min && last >= mid_max && first <= last {
            if !mid_sorted {
                return 1;
            } else {
                return 0;
            }
        } else if (first > mid_min && last >= mid_max) || (first <= mid_min && last < mid_max) {
            if first > last {
                return 2;
            }
            return 1;
        } else if last < mid_min && last < first && first > mid_max {
            return 3;
        } else {
            return 2;
        }
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
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_operations(s);
        assert_eq!(actual, expected);
    }
}
