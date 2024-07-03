struct Solution;
/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![1];
        for i in 1..=row_index {
            res.push((res[(i - 1) as usize] as i64 * (row_index - i + 1) as i64 / i as i64) as i32);
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1])
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1])
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::get_row(0), vec![1])
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::get_row(13),
            vec![1, 13, 78, 286, 715, 1287, 1716, 1716, 1287, 715, 286, 78, 13, 1]
        )
    }
}
