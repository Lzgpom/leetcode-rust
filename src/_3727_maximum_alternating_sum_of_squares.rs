//! Solution for https://leetcode.com/problems/maximum-alternating-sum-of-squares
//! 3727. Maximum Alternating Sum of Squares

impl Solution {
    pub fn max_alternating_sum(mut nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let (lesser, median, greater) = nums.select_nth_unstable_by_key(n / 2, |&num| num.abs());

        return (*median as i64).pow(2)
            + greater
                .iter()
                .map(|value| (*value as i64).pow(2))
                .sum::<i64>()
            - lesser
                .iter()
                .map(|value| (*value as i64).pow(2))
                .sum::<i64>();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], 12)]
    #[case(vec![1,-1,2,-2,3,-3], 16)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i64) {
        let actual = Solution::max_alternating_sum(nums);
        assert_eq!(actual, expected);
    }
}
