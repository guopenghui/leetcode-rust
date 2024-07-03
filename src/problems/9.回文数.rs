struct Solution;
/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || x != 0 && x % 10 == 0 {
            return false;
        }

        // if x < 10 {
        //     return true;
        // }

        let (mut x, mut y) = (x, 0);
        let mut flag = true;

        loop {
            if flag {
                y = y * 10 + x % 10;
            } else {
                x /= 10;
            }

            if x <= y {
                break;
            }
            flag = !flag;
        }

        x == y
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trivial() {
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome(3), true);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome(123), false);
    }
}
