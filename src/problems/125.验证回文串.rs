struct Solution;
/*
 * @lc app=leetcode.cn id=125 lang=rust
 *
 * [125] 验证回文串
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars = s
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())
            .map(|c| c.to_ascii_lowercase());
        let forward = chars.clone();
        let backward = chars.rev();

        forward.zip(backward).all(|(x, y)| x == y)
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
            true
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::is_palindrome("0".to_string()), true);
    }

    #[test]
    fn test_4() {
        assert_eq!(Solution::is_palindrome(" ".to_string()), true);
    }
}
