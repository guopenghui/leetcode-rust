struct Solution;
/*
 * @lc app=leetcode.cn id=12 lang=rust
 *
 * [12] 整数转罗马数字
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut res = Vec::<char>::new();

        while num >= 1000 {
            res.push('M');
            num -= 1000;
        }
        if num >= 900 {
            res.push('C');
            res.push('M');
            num -= 900;
        }
        if num >= 500 {
            res.push('D');
            num -= 500;
        }
        if num >= 400 {
            res.push('C');
            res.push('D');
            num -= 400;
        }
        while num >= 100 {
            res.push('C');
            num -= 100;
        }
        if num >= 90 {
            res.push('X');
            res.push('C');
            num -= 90;
        }
        if num >= 50 {
            res.push('L');
            num -= 50;
        }
        if num >= 40 {
            res.push('X');
            res.push('L');
            num -= 40;
        }
        while num >= 10 {
            res.push('X');
            num -= 10;
        }
        if num >= 9 {
            res.push('I');
            res.push('X');
            num -= 9;
        }
        if num >= 5 {
            res.push('V');
            num -= 5;
        }
        if num >= 4 {
            res.push('I');
            res.push('V');
            num -= 4;
        }
        while num >= 1 {
            res.push('I');
            num -= 1;
        }

        res.into_iter().collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::int_to_roman(1), "I");
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(12), "XII");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
