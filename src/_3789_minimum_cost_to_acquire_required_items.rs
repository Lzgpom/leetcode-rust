//! Solution for https://leetcode.com/problems/minimum-cost-to-acquire-required-items
//! 3789. Minimum Cost to Acquire Required Items

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let mut result = 0i64;
        let mut need1 = need1;
        let mut need2 = need2;

        if cost_both < cost1 + cost2 {
            let needed_both = need1.min(need2);
            need1 -= needed_both;
            need2 -= needed_both;

            result += cost_both as i64 * needed_both as i64;
        }

        result += cost1.min(cost_both) as i64 * need1 as i64;
        result += cost2.min(cost_both) as i64 * need2 as i64;

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
    #[case(3, 2, 1, 3, 2, 3)]
    #[case(5, 4, 15, 2, 3, 22)]
    #[case(5, 4, 15, 0, 0, 0)]
    fn case(
        #[case] cost1: i32,
        #[case] cost2: i32,
        #[case] cost_both: i32,
        #[case] need1: i32,
        #[case] need2: i32,
        #[case] expected: i64,
    ) {
        let actual = Solution::minimum_cost(cost1, cost2, cost_both, need1, need2);
        assert_eq!(actual, expected);
    }
}
