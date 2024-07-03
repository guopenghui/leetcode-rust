struct Solution;
/*
 * @lc app=leetcode.cn id=87 lang=rust
 *
 * [87] 扰乱字符串
 */

// @lc code=start
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let s1: Vec<char> = s1.chars().collect();
        let s2: Vec<char> = s2.chars().collect();
        let n = s1.len();
        let mut dp = vec![vec![vec![false; n]; n]; n];

        for i in 0..n {
            for j in 0..n {
                dp[0][i][j] = s1[i] == s2[j];
            }
        }

        for len in 2..=n {
            for i in 0..=(n - len) {
                for j in 0..=(n - len) {
                    dp[len - 1][i][j] = Self::_is_scramle(i, j, len, &dp);
                }
            }
        }
        dp[n - 1][0][0]
    }

    fn _is_scramle(b1: usize, b2: usize, len: usize, dp: &Vec<Vec<Vec<bool>>>) -> bool {
        (1..len)
            .map(|i| {
                dp[i - 1][b1][b2] && dp[len - i - 1][b1 + i][b2 + i]
                    || dp[i - 1][b1][b2 + (len - i)] && dp[len - i - 1][b1 + i][b2]
            })
            .any(|x| x)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    fn timing(f: impl FnOnce(), des: &str) {
        use std::time::Instant;
        let start = Instant::now();
        let _ = f();
        println!("{} time: {:?}", des, start.elapsed());
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_scramble("great".to_string(), "rgeat".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::is_scramble("abcde".to_string(), "caebd".to_string()),
            false
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::is_scramble("a".to_string(), "a".to_string()),
            true
        );
    }

    #[test]
    fn test_4() {
        let s1: String = ('a'..='z').collect();
        let s2: String = ('a'..='z').rev().collect();
        timing(
            move || {
                assert_eq!(Solution::is_scramble(s1, s2), true);
            },
            "long repeat string",
        )
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::is_scramble("ab".to_string(), "aa".to_string()),
            false
        );
    }
}
