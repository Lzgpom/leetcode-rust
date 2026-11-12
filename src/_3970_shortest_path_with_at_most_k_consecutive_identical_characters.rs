//! Solution for https://leetcode.com/problems/shortest-path-with-at-most-k-consecutive-identical-characters
//! 3970. Shortest Path With At Most K Consecutive Identical Characters

use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

struct Path {
    node: i32,
    weight: i32,
    char: u8,
    char_count: i32,
    visited: HashSet<i32>,
}

impl Path {
    fn new(node: i32, weight: i32, char: u8, char_count: i32, visited: &HashSet<i32>) -> Self {
        let mut visited = visited.clone();
        visited.insert(node);

        Self {
            node,
            weight,
            char,
            char_count,
            visited,
        }
    }
}

struct Edge {
    node: i32,
    weight: i32,
}

impl Edge {
    fn new(node: i32, weight: i32) -> Self {
        Self { node, weight }
    }
}

impl Solution {
    pub fn shortest_path(n: i32, edges: Vec<Vec<i32>>, labels: String, k: i32) -> i32 {
        let chars = labels.as_bytes();
        let graph = edges
            .iter()
            .map(|edge| (edge[0], Edge::new(edge[1], edge[2])))
            .into_group_map();

        let mut min_result: Option<i32> = None;
        let mut queue = VecDeque::new();
        queue.push_back(Path::new(0, 0, chars[0], 1, &HashSet::new()));

        while let Some(path) = queue.pop_front() {
            if path.node == n - 1 {
                min_result = Some(min_result.unwrap_or(i32::MAX).min(path.weight));
                continue;
            }

            if path.weight > min_result.unwrap_or(i32::MAX) {
                continue;
            }

            if let Some(edges) = graph.get(&path.node) {
                for edge in edges {
                    if path.visited.contains(&edge.node) {
                        continue;
                    }

                    let next_char = chars[edge.node as usize];
                    let char_count = if path.char == next_char {
                        path.char_count + 1
                    } else {
                        1
                    };

                    if char_count > k {
                        continue;
                    }

                    queue.push_back(Path::new(
                        edge.node,
                        path.weight + edge.weight,
                        next_char,
                        char_count,
                        &path.visited,
                    ));
                }
            }
        }

        return min_result.unwrap_or(-1);
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
