//! Solution for https://leetcode.com/problems/sum-of-sortable-integers
//! 3886. Sum of Sortable Integers

use itertools::Itertools;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        return (1..=len)
            .into_iter()
            .filter(|k| len % k == 0)
            .filter(|k| Self::check_k(&nums, k))
            .map(|k| k as i32)
            .sum();
    }

    fn check_k(nums: &Vec<i32>, k: &usize) -> bool {
        let chunks: Vec<Vec<i32>> = nums
            .iter()
            .chunks(*k)
            .into_iter()
            .map(|chunk| chunk.map(|it| *it).collect())
            .collect();

        let all_chunks_sortable = chunks
            .iter()
            .all(|chunk| Self::is_chunk_sortable(chunk.iter()));

        let all_chunks_sequential = Self::are_chunks_sequential(chunks.iter());

        return all_chunks_sequential && all_chunks_sortable;
    }

    fn is_chunk_sortable<'a, I>(chunk: I) -> bool
    where
        I: Iterator<Item = &'a i32>,
    {
        let mut iter = chunk.into_iter();
        let first = match iter.next() {
            Some(val) => *val,
            None => return true,
        };

        let mut drops = 0;
        let mut previous = first;
        for &num in iter {
            if num < previous {
                drops += 1;
            }

            previous = num;
        }

        if previous > first {
            drops += 1;
        }

        return drops <= 1;
    }

    fn are_chunks_sequential<'a, I>(chunks: I) -> bool
    where
        I: Iterator<Item = &'a Vec<i32>>,
    {
        let mut previous_max = i32::MIN;

        for chunk in chunks {
            let mut chunk_min = i32::MAX;
            let mut chunk_max = i32::MIN;

            for &num in chunk {
                chunk_min = chunk_min.min(num);
                chunk_max = chunk_max.max(num);
            }

            if chunk_min < previous_max {
                return false;
            }

            previous_max = chunk_max;
        }

        return true;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,2], 3)]
    #[case(vec![7,6,5], 0)]
    #[case(vec![1,2,3], 4)]
    #[case(vec![5,8], 3)]
    #[case(vec![12,18,14], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::sortable_integers(nums);
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(vec![3,1,2,4], false)]
    #[case(vec![3,1,2], true)]
    #[case(vec![12,18,14], false)]
    fn test_is_chunk_sortable(#[case] nums: Vec<i32>, #[case] expected: bool) {
        let actual = Solution::is_chunk_sortable(nums.iter());
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(vec![vec![3,1], vec![2,4]], false)]
    #[case(vec![vec![2,1], vec![2,4]], true)]
    fn test_are_chunks_sequential(#[case] chunks: Vec<Vec<i32>>, #[case] expected: bool) {
        let actual = Solution::are_chunks_sequential(chunks.iter());
        assert_eq!(actual, expected);
    }
}
