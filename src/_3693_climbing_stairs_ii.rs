//! Solution for https://leetcode.com/problems/climbing-stairs-ii
//! 3693. Climbing Stairs II

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut results = vec![i32::MAX; n + 1];
        results[0] = 0;

        for i in 0..=n {
            for j in i + 1..=(i + 3).min(n) {
                let val = results[i] + costs[j - 1] + (j as i32 - i as i32).pow(2);
                results[j] = results[j].min(val);
            }
        }

        return results[n];
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(4, vec![1,2,3,4], 13)]
    #[case(4, vec![5,1,6,2], 11)]
    #[case(3, vec![9,8,3], 12)]
    fn case(#[case] n: i32, #[case] costs: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::climb_stairs(n, costs);
        assert_eq!(actual, expected);
    }
}
