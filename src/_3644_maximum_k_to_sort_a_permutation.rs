//! Solution for https://leetcode.com/problems/maximum-k-to-sort-a-permutation
//! 3644. Maximum K to Sort a Permutation

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        if nums.is_sorted() {
            return 0;
        }

        return nums.iter().enumerate().fold(
            i32::MAX,
            |k, (i, &num)| {
                if i as i32 != num { k & num } else { k }
            },
        );
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,3,2,1], 1)]
    #[case(vec![0,1,3,2], 2)]
    #[case(vec![3,2,1,0], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sort_permutation(nums);
        assert_eq!(actual, expected);
    }
}
