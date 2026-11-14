//! Solution for https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal
//! 3505. Minimum Operations to Make Elements Within K Subarrays Equal

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let x = x as usize;
        let k = k as usize;
        let windows: Vec<i64> = nums
            .windows(x)
            .map(|window| Self::operation_in_slice(window))
            .collect();

        return Self::min_windows(&windows, x, k).unwrap_or(0);
    }

    fn operation_in_slice(nums: &[i32]) -> i64 {
        let mut vec = nums.to_vec();
        let (lesser, median, greater) = vec.select_nth_unstable(nums.len() / 2);
        return lesser
            .iter()
            .map(|&val| *median as i64 - val as i64)
            .sum::<i64>()
            + greater
                .iter()
                .map(|&val| val as i64 - *median as i64)
                .sum::<i64>();
    }

    fn min_windows(windows: &[i64], x: usize, k: usize) -> Option<i64> {
        if k == 1 {
            return windows.iter().min().map(|&it| it);
        }
        if x > windows.len() {
            return None;
        }

        let mut min = None;

        for i in 0..windows.len() - x {
            let current = windows[i];
            if current > min.unwrap_or(i64::MAX) {
                continue;
            }
            min = match Self::min_windows(&windows[i + x..], x, k - 1) {
                Some(val) => {
                    let total_val = current + val;
                    match min {
                        Some(min_val) => Some(total_val.min(min_val)),
                        None => Some(total_val),
                    }
                }
                None => min,
            };
        }

        return min;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,-2,1,3,7,3,6,4,-1], 3, 2, 8)]
    #[case(vec![9,-2,-2,-2,1,5], 2, 2, 3)]
    #[case(vec![-16,18,-4,-2,-18,15], 2, 3, 69)]
    fn case(#[case] nums: Vec<i32>, #[case] x: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::min_operations(nums, x, k);
        assert_eq!(actual, expected);
    }
}
