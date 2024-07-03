/*
 * @lc app=leetcode.cn id=3 lang=rust
 *
 * [3] 无重复字符的最长子串
 */

// @lc code=start
struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut mem = [-1; 128];
        let mut last = 0;
        let mut ans = 0;

        for (i, c) in s.chars().enumerate() {
            if mem[c as usize] != -1 && mem[c as usize] >= last {
                last = mem[c as usize] + 1;
            }
            mem[c as usize] = i as i32;
            ans = ans.max(i as i32 - last + 1);
        }

        ans
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base_test() {
        crate::batch_test! {
            Solution::length_of_longest_substring:
            ("abcabcbb".into()) => 3,
            ("bbbbb".into()) => 1,
            ("pwwkew".into()) => 3,
            ("".into()) => 0,
        }
    }
}
