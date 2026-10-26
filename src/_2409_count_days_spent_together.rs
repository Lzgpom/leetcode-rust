//! Solution for https://leetcode.com/problems/count-days-spent-together
//! 2409. Count Days Spent Together

use std::ops::Sub;

const DAYS_PER_MONTH: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Date {
    month: i32,
    day: i32,
}

impl Date {
    fn new(date: &String) -> Option<Date> {
        let mut iter = date.split("-");
        let month = iter.next().and_then(|it| it.parse::<i32>().ok());
        let day = iter.next().and_then(|it| it.parse::<i32>().ok());

        match (day, month) {
            (Some(day), Some(month)) => Some(Date { day, month }),
            _ => None,
        }
    }

    fn month_index(&self) -> usize {
        return (self.month - 1) as usize;
    }
}

impl Sub for Date {
    type Output = i32;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.month < rhs.month {
            0
        } else if self.month == rhs.month {
            if self.day < rhs.day {
                0
            } else {
                self.day - rhs.day + 1
            }
        } else {
            let days_from_start = self.day;
            let days_till_end = DAYS_PER_MONTH[rhs.month_index()] - rhs.day;
            let days_of_months_between: i32 = if self.month_index() > rhs.month_index() + 1 {
                DAYS_PER_MONTH[rhs.month_index() + 1..self.month_index()]
                    .iter()
                    .sum()
            } else {
                0
            };

            days_till_end + days_from_start + days_of_months_between + 1
        }
    }
}

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        if let Some(days) =
            Self::count_day_together_optional(arrive_alice, leave_alice, arrive_bob, leave_bob)
        {
            return days;
        }
        return 0;
    }

    fn count_day_together_optional(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> Option<i32> {
        let arrive_alice = Date::new(&arrive_alice)?;
        let leave_alice = Date::new(&leave_alice)?;
        let arrive_bob = Date::new(&arrive_bob)?;
        let leave_bob = Date::new(&leave_bob)?;

        let start = arrive_alice.max(arrive_bob);
        let end = leave_alice.min(leave_bob);
        return Some(end - start);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("08-15", "08-18", "08-16", "08-19", 3)]
    #[case("10-01", "10-31", "11-01", "12-31", 0)]
    #[case("10-20", "12-22", "06-21", "07-05", 0)]
    #[case("03-05", "07-14", "04-14", "09-21", 92)]
    #[case("04-20", "06-18", "04-12", "12-19", 60)]
    fn case(
        #[case] arrive_alice: String,
        #[case] leave_alice: String,
        #[case] arrive_bob: String,
        #[case] leave_bob: String,
        #[case] expected: i32,
    ) {
        let actual =
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob);
        assert_eq!(actual, expected);
    }
}
