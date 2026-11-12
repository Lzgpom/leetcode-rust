//! Solution for https://leetcode.com/problems/shortest-path-with-at-most-k-consecutive-identical-characters
//! 3970. Shortest Path With At Most K Consecutive Identical Characters

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;

struct Edge {
    node: usize,
    weight: i32,
}

impl Edge {
    fn new(node: usize, weight: i32) -> Self {
        Self { node, weight }
    }
}

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let chars = labels.as_bytes();
        let graph = edges
            .iter()
            .map(|edge| (edge[0] as usize, Edge::new(edge[1] as usize, edge[2])))
            .into_group_map();

        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();
        let mut distances = vec![vec![i32::MAX; k + 1]; n];
        distances[0][1] = 0;
        heap.push(Reverse((0, 0, 1)));

        while let Some(Reverse((distance, node, count))) = heap.pop() {
            if distance > distances[node][count] {
                continue;
            }

            if node == n - 1 {
                return distance;
            }

            if let Some(edges) = graph.get(&node) {
                for edge in edges {
                    let new_count = if chars[edge.node] == chars[node] {
                        count + 1
                    } else {
                        1
                    };

                    if new_count > k {
                        continue;
                    }

                    let new_distance = distance + edge.weight;
                    if distances[edge.node][new_count] > new_distance {
                        distances[edge.node][new_count] = new_distance;
                        heap.push(Reverse((new_distance, edge.node, new_count)));
                    }
                }
            }
        }

        return -1;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(3, vec![vec![0,1,1],vec![1,2,1],vec![0,2,3]], "aab", 1, 3)]
    #[case(3, vec![vec![0,1,1],vec![1,2,1],vec![0,2,3]], "aab", 2, 2)]
    #[case(3, vec![vec![0,1,1],vec![1,2,1]], "aaa", 2, -1)]
    #[case(3, vec![vec![0,1,1802],vec![1,0,4223],vec![2,0,1537]], "gki", 1, -1)]
    fn case(
        #[case] n: i32,
        #[case] edges: Vec<Vec<i32>>,
        #[case] labels: String,
        #[case] k: i32,
        #[case] expected: i32,
    ) {
        let actual = Solution::shortest_path(n, edges, labels, k);
        assert_eq!(actual, expected);
    }
}
