//! Solution for https://leetcode.com/problems/maximize-sum-of-squares-of-digits
//! 3723. Maximize Sum of Squares of Digits

impl Solution {
    pub fn max_sum_of_squares(num: i32, sum: i32) -> String {
        let nine_digits = sum / 9;
        let other_digit = sum % 9;

        let zero_digits = num - (nine_digits + if other_digit > 0 { 1 } else { 0 });
        if zero_digits < 0 {
            return "".into();
        }

        let mut result = String::with_capacity(num as usize);
        for _ in 0..nine_digits {
            result.push('9');
        }

        if other_digit > 0 {
            result.push_str(&other_digit.to_string());
        }

        for _ in 0..zero_digits {
            result.push('0');
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
    #[case(2, 3, "30")]
    #[case(2, 17, "98")]
    #[case(1, 10, "")]
    fn case(#[case] num: i32, #[case] sum: i32, #[case] expected: String) {
        let actual = Solution::max_sum_of_squares(num, sum);
        assert_eq!(actual, expected);
    }
}
