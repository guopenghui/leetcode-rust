struct Solution;
/*
 * @lc app=leetcode.cn id=191 lang=rust
 *
 * [191] 位1的个数
 */

// @lc code=start
impl Solution {
    #[allow(non_snake_case)]
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut res, mut n) = (0, n);
        while n != 0 {
            res += 1;
            n &= n - 1;
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::hammingWeight(0b00000000_00000000_00000000_00001011),
            3
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::hammingWeight(0b00000000_00000000_00000000_10000000),
            1
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::hammingWeight(0b11111111_11111111_111111111_1111101),
            31
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            Solution::hammingWeight(0b00000000_00000000_00000000_00000000),
            0
        );
    }
}
