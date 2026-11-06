//! Solution for https://leetcode.com/problems/find-the-distance-value-between-two-arrays
//! 1385. Find the Distance Value Between Two Arrays

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, arr2: Vec<i32>, d: i32) -> i32 {
        return arr1
            .iter()
            .filter(|&val1| !arr2.iter().any(|&val2| (val1 - val2).abs() <= d))
            .count() as i32;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![4,5,8], vec![10,9,1,8], 2, 2)]
    #[case(vec![1,4,2,3], vec![-4,-3,6,10,20,30], 3, 2)]
    #[case(vec![2,1,100,3], vec![-5,-2,10,-3,7], 6, 1)]
    fn case(#[case] arr1: Vec<i32>, #[case] arr2: Vec<i32>, #[case] d: i32, #[case] expected: i32) {
        let actual = Solution::find_the_distance_value(arr1, arr2, d);
        assert_eq!(actual, expected);
    }
}
