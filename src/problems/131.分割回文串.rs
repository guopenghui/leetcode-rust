struct Solution;
/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */

// @lc code=start
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        part_str(s.as_str())
            .iter()
            .map(|v| v.iter().map(|s| s.to_string()).collect())
            .collect()
    }
}

fn part_str(s: &str) -> Vec<Vec<&str>> {
    if s.len() == 0 {
        return vec![vec![]];
    }

    let mut res = Vec::new();
    for i in 1..=s.len() {
        let tail = &s[s.len() - i..s.len()];
        if is_huiwen(tail) {
            let mut rest = part_str(&s[0..s.len() - i]);
            for v in rest.iter_mut() {
                v.push(tail);
            }
            res.append(&mut rest);
        }
    }

    res
}

fn is_huiwen(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}
// @lc code=end

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::partition("aab".to_string()),
            vec![vec!["a", "a", "b"], vec!["aa", "b"]]
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::partition("".to_string()),
            vec![Vec::<String>::new()]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::partition("a".to_string()), vec![vec!["a"]]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::partition("ab".to_string()), vec![vec!["a", "b"]]);
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::partition("aba".to_string()),
            vec![vec!["a", "b", "a"], vec!["aba"]]
        );
    }
}
