//! Solution for https://leetcode.com/problems/minimum-moves-to-equal-array-elements-ii
//! 462. Minimum Moves to Equal Array Elements II

impl Solution {
    pub fn min_moves2(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let middle = nums.len() / 2;
        let (lower, median, higher) = nums.select_nth_unstable(middle);

        return lower.iter().map(|&num| *median - num).sum::<i32>()
            + higher.iter().map(|&num| num - *median).sum::<i32>();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3], 2)]
    #[case(vec![1,10,2,9], 16)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_moves2(nums);
        assert_eq!(actual, expected);
    }
}
