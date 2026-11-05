//! Solution for https://leetcode.com/problems/minimum-moves-to-equal-array-elements
//! 453. Minimum Moves to Equal Array Elements

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap_or(&0);
        return nums.iter().map(|num| num - min).sum();
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
