//! Solution for https://leetcode.com/problems/minimum-swaps-to-sort-by-digit-sum
//! 3551. Minimum Swaps to Sort by Digit Sum

use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
struct Num {
    value: i32,
    digit_sum: i32,
}

impl Num {
    pub fn new(value: i32) -> Num {
        Num {
            value,
            digit_sum: Self::sum_digits(value),
        }
    }

    fn sum_digits(value: i32) -> i32 {
        let mut current = value;
        let mut sum = 0;
        while current > 0 {
            let digit = current % 10;
            sum += digit;

            current /= 10;
        }

        return sum;
    }
}

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let num_index: HashMap<i32, usize> = nums
            .iter()
            .map(|value| Num::new(*value))
            .sorted_by(|a, b| {
                let sum_cmp = a.digit_sum.cmp(&b.digit_sum);
                if sum_cmp == Ordering::Equal {
                    a.value.cmp(&b.value)
                } else {
                    sum_cmp
                }
            })
            .enumerate()
            .map(|(index, num)| (num.value, index))
            .collect();

        println!("{:?}", num_index);

        let mut swaps = 0;
        let mut nums = nums;
        for i in 0..nums.len() {
            let mut num = nums[i];
            let mut new_index = num_index[&num];

            while i != new_index {
                let tmp = nums[new_index];
                nums[new_index] = num;
                nums[i] = tmp;

                swaps += 1;

                num = tmp;
                new_index = num_index[&num];
            }
        }

        return swaps;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![37,100], 1)]
    #[case(vec![22,14,33,7], 0)]
    #[case(vec![18,43,34,16], 2)]
    #[case(vec![978678783,989995084,465932830,37255967], 3)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_swaps(nums);
        assert_eq!(actual, expected);
    }
}
