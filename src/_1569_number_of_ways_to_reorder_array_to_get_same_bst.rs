//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst
//! 1569. Number of Ways to Reorder Array to Get Same BST

const MODULO: i64 = 1000000007;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let table = Self::pascal_table(nums.len());

        return Self::dfs(nums, &table) - 1;
    }

    fn pascal_table(n: usize) -> Vec<Vec<i64>> {
        let mut table = vec![vec![0_i64; n]; n];

        for i in 0..n {
            table[i][0] = 1;
            table[i][i] = 1;
        }

        for i in 2..n {
            for j in 1..i {
                table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MODULO;
            }
        }

        return table;
    }

    fn dfs(tree: Vec<i32>, table: &Vec<Vec<i64>>) -> i32 {
        if tree.len() < 3 {
            return 1;
        }

        let root = tree[0];
        let (left, right): (Vec<i32>, Vec<i32>) = tree.iter().skip(1).partition(|num| **num < root);
        let left_count = left.len();
        let left_ways = Self::dfs(left, table) as i64;
        let right_ways = Self::dfs(right, table) as i64;
        let p = table[tree.len() - 1][left_count];
        let ways = (((left_ways * right_ways) % MODULO) * p) % MODULO;

        return ways as i32;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![2,1,3], 1)]
    #[case(vec![3,4,5,1,2], 5)]
    #[case(vec![4,2,1,3,5], 7)]
    #[case(vec![1,2,3], 0)]
    #[case(vec![9,4,2,1,3,6,5,7,8,14,11,10,12,13,16,15,17,18], 216212978)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::num_of_ways(nums);
        assert_eq!(actual, expected);
    }
}
