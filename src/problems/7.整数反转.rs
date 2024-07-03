struct Solution;
/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN {
            return 0;
        }

        let sig = x.signum();
        let rev_iter = IntReverse { rem: x.abs() };
        let mut res = 0;

        const MAX_10: i32 = i32::MAX / 10;
        for digit in rev_iter {
            if res > MAX_10 || res == MAX_10 && digit > 7 {
                return 0;
            }

            res = res * 10 + digit;
        }

        res * sig
    }
}

struct IntReverse {
    rem: i32,
}

impl Iterator for IntReverse {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rem == 0 {
            return None;
        }

        let unit = self.rem % 10;
        self.rem = self.rem / 10;
        Some(unit)
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn it_works2() {
        assert_eq!(Solution::reverse(-123), -321);
    }

    #[test]
    fn it_works3() {
        assert_eq!(Solution::reverse(120), 21);
    }

    #[test]
    fn it_works4() {
        assert_eq!(Solution::reverse(1534236469), 0);
    }

    #[test]
    fn it_works_for_edge_cases() {
        assert_eq!(Solution::reverse(i32::MIN), 0);
        assert_eq!(Solution::reverse(i32::MAX), 0);
        assert_eq!(Solution::reverse(0), 0);
    }
}
