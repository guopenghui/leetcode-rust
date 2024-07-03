struct Solution;
/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let chars = strs.iter().map(|s| s.chars()).collect();
        Extract(chars).collect()
    }
}

struct Extract<'a>(Vec<std::str::Chars<'a>>);

impl Iterator for Extract<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let Self(strs) = self;

        strs.iter_mut().try_fold('*', |res, iter| {
            if let Some(ch) = iter.next() {
                if res == ch || res == '*' {
                    Some(ch)
                } else {
                    None
                }
            } else {
                None
            }
        })
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            String::from("fl")
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ]),
            String::from("")
        )
    }
}
