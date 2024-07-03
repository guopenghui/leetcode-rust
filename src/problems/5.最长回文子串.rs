struct Solution;
/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        // 以上一字符为终点的回文串的可能长度
        let mut lens = vec![];
        let s: Vec<char> = s.chars().collect();
        let mut longest: &[char] = &s[0..1];

        for (i, c) in s.iter().enumerate() {
            // 以当前字符为终点的回文串的可能长度
            let mut new_lens = vec![];
            for len in lens {
                if i - len > 0 && *c == s[i - len - 1] {
                    new_lens.push(len + 2);
                    if longest.len() < len + 2 {
                        longest = &s[i - len - 1..=i];
                    }
                }
            }

            new_lens.push(1);
            new_lens.push(0);
            lens = new_lens;
        }

        longest.iter().collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_palindrome("babad".to_string()),
            "bab".to_string()
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_palindrome("cbbd".to_string()),
            "bb".to_string()
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::longest_palindrome("a".to_string()),
            "a".to_string()
        )
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::longest_palindrome("ac".to_string()),
            "a".to_string()
        )
    }

    #[test]
    fn test_5() {
        assert_eq!(
            Solution::longest_palindrome("bb".to_string()),
            "bb".to_string()
        )
    }

    #[test]
    fn test_6() {
        assert_eq!(
            Solution::longest_palindrome("cbbcc".to_string()),
            "cbbc".to_string()
        )
    }

    #[test]
    fn test_7() {
        assert_eq!(
            Solution::longest_palindrome("bacabab".to_string()),
            "bacab".to_string()
        )
    }
}
