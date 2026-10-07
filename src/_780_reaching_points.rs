//! Solution for https://leetcode.com/problems/reaching-points
//! 780. Reaching Points

impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut tx = tx;
        let mut ty = ty;

        while tx > sx && ty > sy {
            if tx > ty {
                tx %= ty;
            } else {
                ty %= tx;
            }
        }

        return (sx == tx && sy <= ty && (ty - sy) % sx == 0)
            || (sy == ty && sx <= tx && (tx - sx) % sy == 0);
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(1, 1, 3, 5, true)]
    #[case(1, 1, 2, 2, false)]
    #[case(1, 1, 1, 1, true)]
    #[case(9, 10, 9, 19, true)]
    fn case(
        #[case] sx: i32,
        #[case] sy: i32,
        #[case] tx: i32,
        #[case] ty: i32,
        #[case] expected: bool,
    ) {
        let actual = Solution::reaching_points(sx, sy, tx, ty);
        assert_eq!(actual, expected);
    }
}
