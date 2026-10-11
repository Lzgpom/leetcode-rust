//! Solution for https://leetcode.com/problems/minimum-sum-of-squared-difference
//! 2333. Minimum Sum of Squared Difference

use std::cmp::{max, min};

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut diffs = Vec::with_capacity(nums1.len());
        let mut max_diff = 0;

        for (num1, num2) in nums1.iter().zip(nums2) {
            let diff = (num1 - num2).abs();
            diffs.push(diff);

            max_diff = max(max_diff, diff);
        }

        let mut k = k1 + k2;
        let mut l = 0;
        let mut r = max_diff;
        let mut target_diff = 0;

        while l <= r {
            let mid = (r + l) / 2;
            let sum: i64 = diffs
                .iter()
                .map(|diff| if diff > &mid { (diff - mid) as i64 } else { 0 })
                .sum();

            if sum <= k as i64 {
                r = mid - 1;
                target_diff = mid;
            } else {
                l = mid + 1;
            }
        }

        for diff in diffs.iter() {
            if *diff > target_diff {
                k -= diff - target_diff
            }
        }

        diffs.sort_by(|a, b| b.cmp(a));

        let mut result: i64 = 0;
        for diff in diffs {
            let mut actual_diff = min(diff, target_diff) as i64;
            if k > 0 && actual_diff > 0 {
                k -= 1;
                actual_diff -= 1;
            }

            result += actual_diff * actual_diff
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
    #[case(vec![1,2,3,4], vec![2,10,20,19], 0, 0, 579)]
    #[case(vec![1,4,10,12], vec![5,8,6,9], 1, 1, 43)]
    fn case(
        #[case] nums1: Vec<i32>,
        #[case] nums2: Vec<i32>,
        #[case] k1: i32,
        #[case] k2: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::min_sum_square_diff(nums1, nums2, k1, k2);
        assert_eq!(actual, expected);
    }
}
