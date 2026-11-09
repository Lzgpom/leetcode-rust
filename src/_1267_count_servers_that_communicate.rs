//! Solution for https://leetcode.com/problems/count-servers-that-communicate
//! 1267. Count Servers that Communicate

#[derive(Clone)]
enum State {
    None,
    NotConnected(usize),
    Connected,
}

struct ServersState {
    row_state: Vec<State>,
    column_state: Vec<State>,
}

impl ServersState {
    fn new(n: usize, m: usize) -> Self {
        Self {
            row_state: vec![State::None; n],
            column_state: vec![State::None; m],
        }
    }

    fn add_server(&mut self, x: usize, y: usize) -> i32 {
        let row_state = &self.row_state[x].clone();
        let column_state = &self.column_state[y];

        let result = match row_state {
            State::Connected => match column_state {
                State::Connected => 1,
                State::NotConnected(x) => {
                    self.row_state[*x] = State::Connected;
                    2
                }
                State::None => 1,
            },
            State::NotConnected(y) => match column_state {
                State::Connected => {
                    self.column_state[*y] = State::Connected;
                    2
                }
                State::NotConnected(x) => {
                    self.row_state[*x] = State::Connected;
                    self.column_state[*y] = State::Connected;
                    3
                }
                State::None => {
                    self.column_state[*y] = State::Connected;
                    2
                }
            },
            State::None => match column_state {
                State::Connected => 1,
                State::NotConnected(x) => {
                    self.row_state[*x] = State::Connected;
                    2
                }
                State::None => 0,
            },
        };

        if result > 0 {
            self.row_state[x] = State::Connected;
            self.column_state[y] = State::Connected;
        } else {
            self.row_state[x] = State::NotConnected(y);
            self.column_state[y] = State::NotConnected(x);
        }

        return result;
    }
}

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut servers_state = ServersState::new(grid.len(), grid[0].len());
        let mut count = 0;

        for x in 0..grid.len() {
            for y in 0..grid[x].len() {
                if grid[x][y] == 1 {
                    count += servers_state.add_server(x, y);
                }
            }
        }

        return count;
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,0],vec![0,1]], 0)]
    #[case(vec![vec![1,0],vec![1,1]], 3)]
    #[case(vec![vec![1,1,0,0],vec![0,0,1,0],vec![0,0,1,0],vec![0,0,0,1]], 4)]
    #[case(vec![vec![0,0,1,0,1],vec![0,1,0,1,0],vec![0,1,1,1,0],vec![1,0,0,1,1],vec![0,0,1,1,0]], 12)]
    fn case(#[case] grid: Vec<Vec<i32>>, #[case] expected: i32) {
        let actual = Solution::count_servers(grid);
        assert_eq!(actual, expected);
    }
}
