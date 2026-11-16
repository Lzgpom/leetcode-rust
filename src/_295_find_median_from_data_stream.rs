//! Solution for https://leetcode.com/problems/find-median-from-data-stream
//! 295. Find Median from Data Stream

use std::{cmp::Reverse, collections::BinaryHeap};

struct MedianFinder {
    max_heap: BinaryHeap<i32>,
    min_heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
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

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let mut median_finder = MedianFinder::new();
        median_finder.add_num(1);
        median_finder.add_num(2);
        assert_eq!(1.5, median_finder.find_median());
        median_finder.add_num(3);
        assert_eq!(2.0, median_finder.find_median());
    }
}
