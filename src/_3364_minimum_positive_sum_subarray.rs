//! Solution for https://leetcode.com/problems/minimum-positive-sum-subarray
//! 3364. Minimum Positive Sum Subarray

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let l = l as usize;
        let r = r as usize;

        return (l..=r)
            .flat_map(|size| {
                nums.windows(size)
                    .map(|window| window.iter().sum())
                    .filter(|&sum| sum > 0)
            })
            .min()
            .unwrap_or(-1);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,-2,1,4], 2, 3, 1)]
    #[case(vec![-2,2,-3,1], 2, 3, -1)]
    #[case(vec![1,2,3,4], 2, 4, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] l: i32, #[case] r: i32, #[case] expected: i32) {
        let actual = Solution::minimum_sum_subarray(nums, l, r);
        assert_eq!(actual, expected);
    }
}
