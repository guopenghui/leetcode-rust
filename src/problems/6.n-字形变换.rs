struct Solution;
/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] N 字形变换
 */

// @lc code=start
impl Solution {
    // author: ObliqueMotion
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzags: Vec<_> = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzags.sort_by_key(|&(row, _)| row);
        zigzags.into_iter().map(|(_, c)| c).collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
    }

    #[test]
    fn test_convert2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
    }

    #[test]
    fn test_convert3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A");
    }

    #[test]
    fn test_convert4() {
        assert_eq!(Solution::convert("ABCDEFG".to_string(), 2), "ACEGBDF");
    }
}
