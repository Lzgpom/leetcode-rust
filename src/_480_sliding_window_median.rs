//! Solution for https://leetcode.com/problems/sliding-window-median
//! 480. Sliding Window Median

struct PriorityQueue<T> {
    values: Vec<T>,
}

impl<T> PriorityQueue<T> {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<T> PriorityQueue<T>
where
    T: Ord,
{
    fn push(&mut self, element: T) {
        match self.values.binary_search(&element) {
            Ok(index) => self.values.insert(index, element),
            Err(index) => self.values.insert(index, element),
        }
    }

    fn remove(&mut self, element: T) {
        if let Ok(index) = self.values.binary_search(&element) {
            self.values.remove(index);
        }
    }
}

impl PriorityQueue<i32> {
    fn median(&self) -> f64 {
        let middle = self.values.len() / 2;
        if self.values.len() % 2 == 0 {
            return (self.values[middle] as f64 + self.values[middle - 1] as f64) / 2.0;
        } else {
            return self.values[middle] as f64;
        }
    }
}

impl Solution {
    pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let mut queue = PriorityQueue::with_capacity(k);
        let mut result = Vec::new();

        for i in 0..k {
            queue.push(nums[i]);
        }
        result.push(queue.median());

        for i in k..nums.len() {
            queue.remove(nums[i - k]);
            queue.push(nums[i]);

            result.push(queue.median());
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
