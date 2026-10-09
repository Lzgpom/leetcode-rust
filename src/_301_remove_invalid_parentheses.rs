//! Solution for https://leetcode.com/problems/remove-invalid-parentheses
//! 301. Remove Invalid Parentheses

use std::vec;

use itertools::Itertools;

#[derive(PartialEq)]
enum Parenthesis {
    Open,
    Close,
}

#[derive(Debug, Clone)]
struct ParenthesisCount {
    open: u8,
    close: u8,
}

impl ParenthesisCount {
    pub fn add(&self, parenthesis: &Parenthesis) -> ParenthesisCount {
        match parenthesis {
            Parenthesis::Open => ParenthesisCount {
                open: self.open + 1,
                close: self.close,
            },
            Parenthesis::Close => ParenthesisCount {
                open: self.open,
                close: self.close + 1,
            },
        }
    }

    pub fn remove(&self, parenthesis: &Parenthesis) -> Option<ParenthesisCount> {
        match parenthesis {
            Parenthesis::Open => {
                if self.open == 0 {
                    None
                } else {
                    Some(ParenthesisCount {
                        open: self.open - 1,
                        close: self.close,
                    })
                }
            }
            Parenthesis::Close => {
                if self.close == 0 {
                    None
                } else {
                    Some(ParenthesisCount {
                        open: self.open,
                        close: self.close - 1,
                    })
                }
            }
        }
    }

    pub fn can_add_close(&self) -> bool {
        return self.open > self.close;
    }

    pub fn is_valid(&self) -> bool {
        return self.open == self.close;
    }

    pub fn is_empty(&self) -> bool {
        return self.open == 0 && self.close == 0;
    }
}

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let to_remove = Self::to_remove_parenthesis(&s);

        Self::recursive(
            "".into(),
            &ParenthesisCount { open: 0, close: 0 },
            &to_remove,
            0,
            &s,
        )
        .into_iter()
        .unique()
        .collect_vec()
    }

    fn to_remove_parenthesis(s: &String) -> ParenthesisCount {
        let mut unclosed_parenthesis: u8 = 0;
        let mut invalid_closing_parenthesis: u8 = 0;

        for char in s.chars() {
            match char {
                '(' => unclosed_parenthesis += 1,
                ')' => {
                    if unclosed_parenthesis == 0 {
                        invalid_closing_parenthesis += 1;
                    } else {
                        unclosed_parenthesis -= 1;
                    }
                }
                _ => (),
            }
        }

        return ParenthesisCount {
            open: unclosed_parenthesis,
            close: invalid_closing_parenthesis,
        };
    }

    fn recursive(
        current: String,
        current_parenthesis: &ParenthesisCount,
        to_remove: &ParenthesisCount,
        index: usize,
        s: &String,
    ) -> Vec<String> {
        let char = match s.chars().nth(index) {
            Some(char) => char,
            None => {
                if current_parenthesis.is_valid() && to_remove.is_empty() {
                    return vec![current];
                } else {
                    return vec![];
                }
            }
        };

        let parenthesis: Option<Parenthesis> = match char {
            '(' => Some(Parenthesis::Open),
            ')' => Some(Parenthesis::Close),
            _ => None,
        };

        if let Some(parenthesis) = parenthesis {
            let mut strings = vec![];

            let is_close_and_cannot_add =
                parenthesis == Parenthesis::Close && !current_parenthesis.can_add_close();
            if !is_close_and_cannot_add {
                let mut new_current = current.clone();
                new_current.push(char);

                let current_parenthesis = current_parenthesis.add(&parenthesis);
                strings.append(&mut Self::recursive(
                    new_current,
                    &current_parenthesis,
                    &to_remove,
                    index + 1,
                    s,
                ));
            }

            if let Some(to_remove) = to_remove.remove(&parenthesis) {
                let mut new_current = current.clone();
                new_current.push(char);

                strings.append(&mut Self::recursive(
                    current.clone(),
                    &current_parenthesis,
                    &to_remove,
                    index + 1,
                    s,
                ));
            }

            return strings;
        } else {
            let mut new_current = current.clone();
            new_current.push(char);

            return Self::recursive(new_current, &current_parenthesis, to_remove, index + 1, s);
        }
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("()())()", vec!["()()()".into(),"(())()".into()])]
    #[case("(a)())()", vec!["(a)()()".into(),"(a())()".into()])]
    #[case(")(", vec!["".into()])]
    fn case(#[case] s: String, #[case] expected: Vec<String>) {
        let actual = Solution::remove_invalid_parentheses(s);
        assert_eq!(actual, expected);
    }
}
