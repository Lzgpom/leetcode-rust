//! Solution for https://leetcode.com/problems/minimum-moves-to-equal-array-elements-iii
//! 3736. Minimum Moves to Equal Array Elements III

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let max = nums.iter().max().unwrap_or(&0);
        return nums.iter().map(|num| max - num).sum();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,3], 3)]
    #[case(vec![4,4,5], 2)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_moves(nums);
        assert_eq!(actual, expected);
    }
}
