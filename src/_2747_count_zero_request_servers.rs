//! Solution for https://leetcode.com/problems/count-zero-request-servers
//! 2747. Count Zero Request Servers

use std::collections::HashMap;
use std::collections::hash_map::Entry;

use itertools::Itertools;

impl Solution {
    pub fn count_servers(n: i32, logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
        let mut sorted_logs = logs;
        sorted_logs.sort_unstable_by_key(|entry| entry[1]);

        let mut results = vec![0; queries.len()];

        let mut servers_in_range = HashMap::new();
        let mut start_i = 0usize;
        let mut end_i = 0usize;

        for (i, query) in queries
            .into_iter()
            .enumerate()
            .sorted_unstable_by_key(|&(_, x)| x)
        {
            let start_range = query - x;
            let end_range = query;

            while end_i < sorted_logs.len() && sorted_logs[end_i][1] <= end_range {
                servers_in_range
                    .entry(sorted_logs[end_i][0])
                    .and_modify(|value| *value += 1)
                    .or_insert(1);
                end_i += 1;
            }

            while start_i < sorted_logs.len() && sorted_logs[start_i][1] < start_range {
                if let Entry::Occupied(mut entry) = servers_in_range.entry(sorted_logs[start_i][0])
                {
                    let val = entry.get_mut();
                    *val -= 1;
                    if *val == 0 {
                        entry.remove_entry();
                    }
                }
                start_i += 1;
            }

            results[i] = n - servers_in_range.keys().count() as i32;
        }

        return results;
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
    #[case(9, vec![vec![6,27],vec![3,6],vec![5,6],vec![2,10],vec![8,15],vec![3,15],vec![5,5],vec![1,30],vec![9,3],vec![8,13]], 4, vec![13,7,10,8,27,18], vec![7,6,6,7,8,7])]
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
