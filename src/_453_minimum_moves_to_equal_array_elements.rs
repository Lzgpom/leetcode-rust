//! Solution for https://leetcode.com/problems/minimum-moves-to-equal-array-elements
//! 453. Minimum Moves to Equal Array Elements

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));

        let mut sum = 0;
        for i in 0..nums.len() - 1 {
            sum += (nums[i] - nums[i + 1]) * (i as i32 + 1)
        }
        return sum;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], 3)]
    #[case(vec![1,1,1], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_moves(nums);
        assert_eq!(actual, expected);
    }
}
