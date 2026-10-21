//! Solution for https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst
//! 1569. Number of Ways to Reorder Array to Get Same BST

const MODULO: i64 = 1000000007;

#[derive(Debug)]
struct Node {
    value: i32,
    pub count: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn tree(values: &Vec<i32>) -> Option<Node> {
        let mut root = match values.get(0) {
            Some(value) => Node::new(*value),
            None => return None,
        };

        for num in values.iter().skip(1) {
            root.insert(*num);
        }

        return Some(root);
    }

    fn new(value: i32) -> Node {
        Node {
            value,
            count: 1,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: i32) {
        self.count += 1;

        if value < self.value {
            match self.left.as_mut() {
                Some(left) => left.insert(value),
                None => self.left = Some(Box::new(Self::new(value))),
            }
        } else if value > self.value {
            match self.right.as_mut() {
                Some(right) => right.insert(value),
                None => self.right = Some(Box::new(Self::new(value))),
            }
        }
    }
}

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let root = match Node::tree(&nums) {
            Some(node) => node,
            None => return 1,
        };

        let table = Self::pascal_table(nums.len());

        return Self::ways_in_node(&Some(Box::new(root)), &table) - 1;
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

    fn ways_in_node(node: &Option<Box<Node>>, table: &Vec<Vec<i64>>) -> i32 {
        let node = match node {
            Some(value) => value,
            None => return 1,
        };

        if node.count == 1 {
            return 1;
        }

        let left = Self::ways_in_node(&node.left, table) as i64;
        let right = Self::ways_in_node(&node.right, table) as i64;

        let left_count = node.left.as_ref().map(|it| it.count).unwrap_or(0);
        let total_count = left_count + node.right.as_ref().map(|it| it.count).unwrap_or(0);
        let combinations = table[total_count][left_count];
        let ways = (((left * right) % MODULO) * combinations) % MODULO;
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
