//! Solution for https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays
//! 689. Maximum Sum of 3 Non-Overlapping Subarrays

struct Subarray {
    index: usize,
    sum: i32,
    indexes: Vec<usize>,
}

impl Subarray {
    fn new(index: usize, sum: i32, indexes: Vec<usize>) -> Self {
        Self {
            index,
            sum,
            indexes,
        }
    }
}

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = 3usize;
        let k = k as usize;
        let windows: Vec<i32> = nums
            .windows(k)
            .map(|window| window.iter().map(|&val| val).sum::<i32>())
            .collect();

        let mut best_subarrays: Vec<Subarray> = Vec::with_capacity(n);
        for i in 0..n {
            let index = i * k;
            let sum = windows[index];
            if i > 0 {
                let prev = &best_subarrays[i - 1];
                let mut indexes = prev.indexes.clone();
                indexes.push(index);
                best_subarrays.push(Subarray::new(index, sum + prev.sum, indexes));
            } else {
                best_subarrays.push(Subarray::new(index, sum, vec![index]));
            }
        }

        while best_subarrays[n - 1].index < windows.len() - 1 {
            for i in 0..n {
                let new_index = best_subarrays[i].index + 1;
                let sum_at_window = windows[new_index];
                best_subarrays[i].index = new_index;
                if i == 0 {
                    if sum_at_window > best_subarrays[i].sum {
                        best_subarrays[i].sum = sum_at_window;
                        best_subarrays[i].indexes[0] = new_index;
                    }
                } else {
                    let sum_with_prev = sum_at_window + best_subarrays[i - 1].sum;
                    if sum_with_prev > best_subarrays[i].sum {
                        best_subarrays[i].sum = sum_with_prev;
                        for j in 0..best_subarrays[i - 1].indexes.len() {
                            best_subarrays[i].indexes[j] = best_subarrays[i - 1].indexes[j];
                        }
                        best_subarrays[i].indexes[i] = new_index;
                    }
                }
            }
        }

        return best_subarrays[n - 1]
            .indexes
            .iter()
            .map(|&it| it as i32)
            .collect();
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,2,1,2,6,7,5,1], 2, vec![0,3,5])]
    #[case(vec![1,2,1,2,1,2,1,2,1], 2, vec![0,2,4])]
    #[case(vec![7,13,20,19,19,2,10,1,1,19], 3, vec![1,4,7])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<i32>) {
        let actual = Solution::max_sum_of_three_subarrays(nums, k);
        assert_eq!(actual, expected);
    }
}
