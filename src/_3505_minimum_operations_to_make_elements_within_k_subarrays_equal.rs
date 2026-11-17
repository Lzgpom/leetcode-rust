//! Solution for https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal
//! 3505. Minimum Operations to Make Elements Within K Subarrays Equal

use std::{
    cmp::Reverse,
    collections::{BTreeMap, btree_map::Entry},
};

struct MultiSet<T> {
    count: usize,
    tree: BTreeMap<T, usize>,
}

impl<T> MultiSet<T> {
    fn new() -> Self {
        Self {
            count: 0,
            tree: BTreeMap::new(),
        }
    }
}

impl<T> MultiSet<T>
where
    T: Ord + Copy,
{
    fn peek(&self) -> Option<&T> {
        self.tree.keys().next_back()
    }

    fn push(&mut self, element: T) {
        self.tree
            .entry(element)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        self.count += 1;
    }

    fn pop(&mut self) -> Option<T> {
        let &val = self.peek()?;
        self.remove(val);
        return Some(val);
    }

    fn remove(&mut self, element: T) -> bool {
        if let Entry::Occupied(mut entry) = self.tree.entry(element) {
            let val = entry.get_mut();
            *val -= 1;
            if *val == 0 {
                entry.remove();
            }
            self.count -= 1;

            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        self.count
    }
}

struct MedianFinder {
    max_heap: MultiSet<i32>,
    min_heap: MultiSet<Reverse<i32>>,
    sum_max: i64,
    sum_min: i64,
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: MultiSet::new(),
            min_heap: MultiSet::new(),
            sum_max: 0,
            sum_min: 0,
        }
    }

    fn add_num(&mut self, num: i32) {
        let goes_max = match (self.max_heap.peek(), self.min_heap.peek()) {
            (Some(&top), _) => num < top,
            (None, Some(&Reverse(bottom))) => num <= bottom,
            (None, None) => true,
        };

        if goes_max {
            self.max_heap.push(num);
            self.sum_max += num as i64;
        } else {
            self.min_heap.push(Reverse(num));
            self.sum_min += num as i64;
        }

        self.balance();
    }

    fn remove_num(&mut self, num: i32) {
        if self.max_heap.remove(num) {
            self.sum_max -= num as i64;
        } else {
            self.min_heap.remove(Reverse(num));
            self.sum_min -= num as i64;
        }
    }

    fn balance(&mut self) {
        while self.max_heap.len() > self.min_heap.len() {
            if let Some(val) = self.max_heap.pop() {
                self.sum_max -= val as i64;
                self.sum_min += val as i64;
                self.min_heap.push(Reverse(val));
            }
        }

        while self.min_heap.len() > self.max_heap.len() {
            if let Some(Reverse(val)) = self.min_heap.pop() {
                self.sum_max += val as i64;
                self.sum_min -= val as i64;
                self.max_heap.push(val);
            }
        }
    }

    fn operations(&self) -> i64 {
        let median = self
            .max_heap
            .peek()
            .map(|&val| val as i64)
            .unwrap_or(i64::MAX);
        let operations_max = median * self.max_heap.len() as i64 - self.sum_max;
        let operations_min = self.sum_min - median * self.min_heap.len() as i64;
        return operations_max + operations_min;
    }
}

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
        let windows = Self::init_windows(&nums, x);

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

    fn init_windows(nums: &[i32], x: usize) -> Vec<i64> {
        let mut median_finder = MedianFinder::new();
        let mut result = Vec::new();

        for i in 0..x {
            median_finder.add_num(nums[i]);
        }
        result.push(median_finder.operations());

        for i in x..nums.len() {
            median_finder.remove_num(nums[i - x]);
            median_finder.add_num(nums[i]);

            result.push(median_finder.operations());
        }

        return result;
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
