//! Solution for https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum
//! 1524. Number of Sub-arrays With Odd Sum

const MODULO: i32 = 1000000007;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        let mut even = 1;
        let mut odd = 0;
        let mut prefix = 0;
        let mut total = 0;

        for num in arr {
            prefix = (1 & num) ^ prefix;
            if prefix == 0 {
                total += odd;
                even += 1;
            } else {
                total += even;
                odd += 1;
            }

            total %= MODULO;
        }

        return total;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,5], 4)]
    #[case(vec![2,4,6], 0)]
    #[case(vec![1,2,3,4,5,6,7], 16)]
    fn case(#[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_of_subarrays(arr);
        assert_eq!(actual, expected);
    }
}
