//! Solution for https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares
//! 2897. Apply Operations on Array to Maximize Sum of Squares

const MODULO: i64 = 1000000007i64;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut num_bits = nums.into_iter().fold(vec![0u32; 31], |mut bits, num| {
            for i in 0..31 {
                if num & (1 << i) != 0 {
                    bits[i] += 1;
                }
            }
            bits
        });

        let mut result = 0i64;

        for _ in 0..k {
            let mut num: i64 = 0;

            for (i, bit) in num_bits.iter_mut().enumerate() {
                if *bit > 0 {
                    num += 1 << i;
                    *bit -= 1;
                }
            }

            result = (result + (num * num)) % MODULO;
        }

        return result as i32;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,6,5,8], 2, 261)]
    #[case(vec![4,5,4,7], 3, 90)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
