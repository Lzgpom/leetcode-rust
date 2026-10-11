//! Solution for https://leetcode.com/problems/sum-of-squares-of-special-elements
//! 2778. Sum of Squares of Special Elements

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        return nums
            .iter()
            .enumerate()
            .filter(|(i, _)| nums.len() % (i + 1) == 0)
            .map(|(_, val)| val.pow(2))
            .sum();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,3,4], 21)]
    #[case(vec![2,7,1,19,18,3], 63)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sum_of_squares(nums);
        assert_eq!(actual, expected);
    }
}
