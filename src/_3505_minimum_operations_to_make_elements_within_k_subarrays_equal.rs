//! Solution for https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal
//! 3505. Minimum Operations to Make Elements Within K Subarrays Equal
struct Subarray {
    index: usize,
    min_operations: i64,
}

impl Subarray {
    fn new(index: usize, min_operations: i64) -> Self {
        Self {
            index,
            min_operations,
        }
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let x = x as usize;
        let k = k as usize;
        let last = k - 1;
        let windows: Vec<i64> = nums
            .windows(x)
            .map(|window| Self::operation_in_slice(window))
            .collect();

        let mut subarrays = Self::init_subarrays(&windows, x, k);

        while subarrays[last].index < windows.len() - 1 {
            for i in 0..k {
                let new_index = subarrays[i].index + 1;
                subarrays[i].index = new_index;

                if i == 0 {
                    if subarrays[i].min_operations > windows[new_index] {
                        subarrays[i].min_operations = windows[new_index];
                    }
                } else {
                    let window_operations = windows[new_index] + subarrays[i - 1].min_operations;
                    if subarrays[i].min_operations > window_operations {
                        subarrays[i].min_operations = window_operations;
                    }
                }
            }
        }

        return subarrays[last].min_operations;
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

    fn init_subarrays(windows: &[i64], x: usize, k: usize) -> Vec<Subarray> {
        let mut best_subarrays: Vec<Subarray> = Vec::with_capacity(k);
        best_subarrays.push(Subarray {
            index: 0,
            min_operations: windows[0],
        });

        for i in 1..k {
            let index = x * i;
            let operations = best_subarrays[i - 1].min_operations + windows[index];
            best_subarrays.push(Subarray::new(index, operations));
        }

        return best_subarrays;
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
    #[case(vec![14,-6,-12,-7], 2, 2, 25)]
    #[case(vec![-16,18,-4,-2,-18,15], 2, 3, 69)]
    fn case(#[case] nums: Vec<i32>, #[case] x: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::min_operations(nums, x, k);
        assert_eq!(actual, expected);
    }
}
