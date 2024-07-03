struct Solution;
/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 */

// @lc code=start
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set = nums.into_iter().collect::<std::collections::HashSet<i32>>();
        let mut max = 0;
        // 用set去重，避免了判断是否是已经处理过某头元素
        // 且用set来进行迭代，相比nums减少了迭代次数
        for n in set.iter() {
            if !set.contains(&(n - 1)) {
                let mut len = 1;
                while set.contains(&(n + len)) {
                    len += 1;
                }
                max = max.max(len);
            }
        }
        max
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    fn timing(f: impl Fn(), des: &str) {
        let start = Instant::now();
        let _ = f();
        println!("{} time: {:?}", des, start.elapsed());
    }

    #[test]
    fn test_0() {
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }

    #[test]
    fn test_1() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]),
            9
        );
    }

    #[test]
    fn test_3() {
        timing(
            || {
                assert_eq!(
                    Solution::longest_consecutive((1..=10).rev().cycle().take(100000).collect()),
                    10
                )
            },
            "",
        );
    }
}
