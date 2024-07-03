struct Solution;
/*
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * [120] 三角形最小路径和
 */

// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; triangle.len() + 1];

        // 反过来求，反而不需要处理复杂的依赖和边界了
        for n in (0..triangle.len()).rev() {
            for i in 0..=n {
                dp[i] = std::cmp::min(dp[i], dp[i + 1]) + triangle[n][i];
            }
        }
        dp[0]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::minimum_total(vec![vec![2]]), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
