//! Solution for https://leetcode.com/problems/minimum-operations-to-sort-a-permutation
//! 3942. Minimum Operations to Sort a Permutation

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        if nums.is_sorted() {
            return 0;
        }

        if !Self::is_sortable(&nums) {
            return -1;
        }

        let index = match nums
            .iter()
            .enumerate()
            .find(|(_, num)| **num == 0 || **num == nums.len() as i32 - 1)
            .map(|(index, _)| index)
        {
            Some(val) => val,
            None => return -1,
        };

        let is_reversed = nums[index] == 0;
        let index = index as i32;

        let operators_to_left = index + 1 + is_reversed.then(|| 1).unwrap_or(0);
        let operation_to_right =
            nums.len() as i32 - index + 1 + is_reversed.then(|| -1).unwrap_or(0);

        return operators_to_left.min(operation_to_right);
    }

    fn is_sortable(nums: &Vec<i32>) -> bool {
        let next_num = nums.iter().cycle().skip(1);
        return nums.iter().zip(next_num).all(|(n1, n2)| {
            let diff = (n1 - n2).abs();
            return diff <= 1 || diff == nums.len() as i32 - 1;
        });
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![0,2,1], 2)]
    #[case(vec![1,0,2], 2)]
    #[case(vec![2,0,1,3], -1)]
    #[case(vec![11,3,13,4,6,8,9,1,7,10,0,14,12,2,5], -1)]
    #[case(vec![2,3,4,5,0,1], 4)]
    #[case(vec![33,32,31,30,29,28,27,26,25,24,23,22,21,20,19,18,17,16,15,14,13,12,11,10,9,8,7,6,5,4,3,2,1,0,67,66,65,64,63,62,61,60,59,58,57,56,55,54,53,52,51,50,49,48,47,46,45,44,43,42,41,40,39,38,37,36,35,34], 35)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_operations(nums);
        assert_eq!(actual, expected);
    }
}
