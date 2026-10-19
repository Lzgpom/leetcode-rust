//! Solution for https://leetcode.com/problems/find-target-indices-after-sorting-array
//! 2089. Find Target Indices After Sorting Array

use itertools::Itertools;

impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        return nums
            .into_iter()
            .sorted_unstable()
            .enumerate()
            .filter(|(_, num)| *num == target)
            .map(|(index, _)| index as i32)
            .collect();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,5,2,3], 2, vec![1,2])]
    #[case(vec![1,2,5,2,3], 3, vec![3])]
    #[case(vec![1,2,5,2,3], 5, vec![4])]
    fn case(#[case] nums: Vec<i32>, #[case] target: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::target_indices(nums, target);
        assert_eq!(actual, expected);
    }
}
