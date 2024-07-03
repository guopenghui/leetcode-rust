struct Solution;
/*
 * @lc app=leetcode.cn id=72 lang=rust
 *
 * [72] 编辑距离
 */

// @lc code=start
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut dp = [[0; 501]; 501];
        for i in 0..=500 {
            dp[i][0] = i;
        }
        for j in 0..500 {
            dp[0][j] = j;
        }

        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                dp[i][j] = if word1[i - 1] == word2[j - 1] {
                    dp[i - 1][j - 1]
                } else {
                    use std::cmp::min;
                    min(min(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]) + 1
                }
            }
        }

        dp[word1.len()][word2.len()] as i32
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }

    fn test_3() {
        assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
        assert_eq!(Solution::min_distance("".to_string(), "cat".to_string()), 3);
        assert_eq!(Solution::min_distance("cat".to_string(), "".to_string()), 3);
        assert_eq!(
            Solution::min_distance("cat".to_string(), "cat".to_string()),
            0
        );
    }
}
