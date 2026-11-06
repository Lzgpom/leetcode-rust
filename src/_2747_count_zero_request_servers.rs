//! Solution for https://leetcode.com/problems/count-zero-request-servers
//! 2747. Count Zero Request Servers

use itertools::Itertools;

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let server_to_logs = logs
            .into_iter()
            .map(|entry| (entry[0], entry[1]))
            .into_group_map();

        let mut result = Vec::with_capacity(queries.len());
        for query in queries {
            let range = query - x..=query;
            let mut count = 0;
            for server_id in 1..=n {
                if let Some(server_logs) = server_to_logs.get(&server_id) {
                    if server_logs.iter().any(|time| range.contains(time)) {
                        count += 1;
                        continue;
                    }
                }
            }

            result.push(n - count);
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
    #[case(3, vec![vec![1,3],vec![2,6],vec![1,5]], 5, vec![10,11], vec![1,2])]
    #[case(3, vec![vec![2,4],vec![2,1],vec![1,2],vec![3,1]], 2, vec![3,4], vec![0,1])]
    fn case(
        #[case] n: i32,
        #[case] logs: Vec<Vec<i32>>,
        #[case] x: i32,
        #[case] queries: Vec<i32>,
        #[case] expected: Vec<i32>,
    ) {
        let actual = Solution::count_servers(n, logs, x, queries);
        assert_eq!(actual, expected);
    }
}
