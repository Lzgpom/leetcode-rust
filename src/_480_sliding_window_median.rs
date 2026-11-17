//! Solution for https://leetcode.com/problems/sliding-window-median
//! 480. Sliding Window Median

use std::collections::btree_map::Entry;
use std::{cmp::Reverse, collections::BTreeMap};

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
}

impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: MultiSet::new(),
            min_heap: MultiSet::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        match self.max_heap.peek() {
            Some(&val) => {
                if num < val {
                    self.max_heap.push(num);
                } else {
                    self.min_heap.push(Reverse(num));
                }
            }
            None => self.max_heap.push(num),
        }

        self.balance();
    }

    fn remove_num(&mut self, num: i32) {
        if !self.max_heap.remove(num) {
            self.min_heap.remove(Reverse(num));
        }
    }

    fn balance(&mut self) {
        while self.max_heap.len() > self.min_heap.len() {
            if let Some(val) = self.max_heap.pop() {
                self.min_heap.push(Reverse(val));
            }
        }

        while self.min_heap.len() > self.max_heap.len() {
            if let Some(Reverse(val)) = self.min_heap.pop() {
                self.max_heap.push(val);
            }
        }
    }

    fn find_median(&self) -> f64 {
        let back = self.max_heap.peek().map(|&val| val as f64).unwrap_or(0.0);
        if self.max_heap.len() != self.min_heap.len() {
            return back;
        } else {
            let front = self
                .min_heap
                .peek()
                .map(|Reverse(val)| *val as f64)
                .unwrap_or(0.0);

            return (back + front) / 2.0;
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut queue = MedianFinder::new();
        let mut result = Vec::new();

        for i in 0..k {
            queue.add_num(nums[i]);
        }
        result.push(queue.find_median());

        for i in k..nums.len() {
            queue.remove_num(nums[i - k]);
            queue.add_num(nums[i]);

            result.push(queue.find_median());
        }

        return result;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000])]
    #[case(vec![1,2,3,4,2,3,1,4,2], 3, vec![2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<f64>) {
        let actual = Solution::median_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
