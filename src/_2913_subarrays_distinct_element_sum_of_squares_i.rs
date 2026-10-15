//! Solution for https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-i
//! 2913. Subarrays Distinct Element Sum of Squares I

use std::collections::HashSet;

impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            let mut unique_numbers = HashSet::new();
            for num in nums.iter().skip(i) {
                unique_numbers.insert(num);
                result += unique_numbers.len().pow(2) as i32;
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
    #[case(vec![1,2,1], 15)]
    #[case(vec![1,1], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sum_counts(nums);
        assert_eq!(actual, expected);
    }
}
