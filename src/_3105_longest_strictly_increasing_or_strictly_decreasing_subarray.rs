//! Solution for https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray
//! 3105. Longest Strictly Increasing or Strictly Decreasing Subarray

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut count = 1;
        let mut result = 0;
        for (num1, num2) in nums.iter().zip(nums.iter().skip(1)) {
            if num1 < num2 {
                count += 1;
            } else {
                result = result.max(count);
                count = 1;
            }
        }

        result = result.max(count);
        count = 1;

        for (num1, num2) in nums.iter().zip(nums.iter().skip(1)) {
            if num1 > num2 {
                count += 1;
            } else {
                result = result.max(count);
                count = 1;
            }
        }

        return result.max(count);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,4,3,3,2], 2)]
    #[case(vec![3,3,3,3], 1)]
    #[case(vec![3,2,1], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::longest_monotonic_subarray(nums);
        assert_eq!(actual, expected);
    }
}
