//! Solution for https://leetcode.com/problems/calculate-digit-sum-of-a-string
//! 2243. Calculate Digit Sum of a String

use itertools::Itertools;

const RADIX: u32 = 10;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let k = k as usize;
        let mut digits: Vec<u32> = s
            .chars()
            .into_iter()
            .map(|char| char.to_digit(RADIX).unwrap())
            .collect();

        while digits.len() > k {
            digits = digits
                .chunks(k)
                .map(|chunk| chunk.iter().sum())
                .flat_map(|sum: u32| {
                    sum.to_string()
                        .chars()
                        .into_iter()
                        .map(|char| char.to_digit(RADIX).unwrap())
                        .collect_vec()
                })
                .collect()
        }

        return digits.iter().map(|digit| digit.to_string()).join("");
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("11111222223", 3, "135")]
    #[case("00000000", 3, "000")]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: String) {
        let actual = Solution::digit_sum(s, k);
        assert_eq!(actual, expected);
    }
}
