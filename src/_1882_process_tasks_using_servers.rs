//! Solution for https://leetcode.com/problems/process-tasks-using-servers
//! 1882. Process Tasks Using Servers

use std::{cmp::Reverse, collections::BinaryHeap};

struct Datacenter {
    available_servers: BinaryHeap<Reverse<Server>>,
    processes: BinaryHeap<Reverse<Process>>,
}

impl Datacenter {
    fn new(servers: Vec<i32>) -> Self {
        Self {
            available_servers: servers
                .iter()
                .enumerate()
                .map(|(index, &weight)| Reverse(Server::new(weight, index as i32)))
                .collect(),
            processes: BinaryHeap::new(),
        }
    }

    fn get_available_server(&mut self) -> Option<Server> {
        return self.available_servers.pop().map(|it| it.0);
    }

    fn add_process(&mut self, process: Process) {
        self.processes.push(Reverse(process));
    }

    fn end_processes(&mut self, end_time: i32) {
        while let Some(Reverse(process)) = self.processes.peek() {
            if process.end_time <= end_time {
                if let Some(Reverse(process)) = self.processes.pop() {
                    self.available_servers.push(Reverse(process.server));
                }
            } else {
                return;
            }
        }
    }

    fn advance_time(&mut self) -> Option<i32> {
        if !self.available_servers.is_empty() {
            return None;
        }

        let time = self.processes.peek()?.0.end_time;
        return Some(time);
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
struct Server {
    weight: i32,
    index: i32,
}

impl Server {
    fn new(weight: i32, index: i32) -> Self {
        Self { weight, index }
    }
}

#[derive(Ord, Eq, PartialOrd, PartialEq, Debug)]
struct Process {
    end_time: i32,
    server: Server,
}

impl Process {
    fn new(end_time: i32, server: Server) -> Self {
        Process { end_time, server }
    }
}

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut datacenter = Datacenter::new(servers);
        let mut time = 0;

        let mut result = Vec::new();
        for (j, task) in tasks.iter().enumerate() {
            time = time.max(j as i32);
            match datacenter.advance_time() {
                Some(end_time) => time = end_time,
                None => (),
            }

            datacenter.end_processes(time);

            match datacenter.get_available_server() {
                Some(server) => {
                    result.push(server.index);
                    datacenter.add_process(Process {
                        end_time: time + task,
                        server,
                    });
                }
                None => (),
            }
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
    #[case(vec![3,3,2], vec![1,2,3,2,1,2], vec![2,2,0,2,1,2])]
    #[case(vec![5,1,4,3,2], vec![2,1,2,4,5,2,1], vec![1,4,1,4,1,3,2])]
    fn case(#[case] servers: Vec<i32>, #[case] tasks: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::assign_tasks(servers, tasks);
        assert_eq!(actual, expected);
    }
}
