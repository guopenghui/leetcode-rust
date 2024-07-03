struct Solution;
/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];

        for _ in 1..num_rows {
            let last = res.last().unwrap();
            let new_level = std::iter::once(&0)
                .chain(last.iter())
                .zip(last.iter().chain(std::iter::once(&0)))
                .map(|(&a, &b)| a + b)
                .collect::<Vec<i32>>();

            res.push(new_level)
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
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
