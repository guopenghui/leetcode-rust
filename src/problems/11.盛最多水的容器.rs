struct Solution;
/*
 * @lc app=leetcode.cn id=11 lang=rust
 *
 * [11] 盛最多水的容器
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        let (mut ls, mut rs): (Vec<(i32, i32)>, Vec<(i32, i32)>) = (vec![], vec![]);

        for (i, &h) in height.iter().enumerate() {
            while let Some(&t) = rs.last() {
                if t.0 > h {
                    break;
                }
                rs.pop();
            }
            rs.push((h, i as i32));
        }
        for (i, &h) in height.iter().rev().enumerate() {
            while let Some(&t) = ls.last() {
                if t.0 > h {
                    break;
                }
                ls.pop();
            }
            ls.push((h, (height.len() - i - 1) as i32));
        }

        let mut res = 0;
        // ls 和 rs 是单调的，最低的同时也在最外面，对于左边最低的，只需要与右边最外面(最低)的比较，反之亦然
        // 这样就不需要检查全部组合了，从 m*n 变为 min(m,n)
        while let (Some(&l), Some(&r)) = (ls.last(), rs.last()) {
            res = max(res, (r.1 - l.1) * min(l.0, r.0));
            if l.0 < r.0 {
                ls.pop();
            } else {
                rs.pop();
            }
        }
        res
    }

    // fn scan(v: Vec<i32>) -> i32 {}
}

// 另一种写法，比较一下时间
// impl Solution {
//     pub fn max_area(height: Vec<i32>) -> i32 {
//         use std::cmp::{max, min};
//         let (mut l, mut r) = (0, height.len() - 1);
//         let mut res = 0;
//         while l < r {
//             res = max(res, (r - l) as i32 * min(height[l], height[r]));
//             if height[l] <= height[r] {
//                 l += 1;
//             } else {
//                 r -= 1;
//             }
//         }
//         res
//     }
// }
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::max_area(vec![0, 1]), 0);
        assert_eq!(Solution::max_area(vec![1, 0]), 0);
        assert_eq!(Solution::max_area(vec![0, 0]), 0);
    }

    #[test]
    fn test_time() {
        assert_eq!(Solution::max_area((1..10000).collect()), 24995000);
        assert_eq!(Solution::max_area((1..10000).rev().collect()), 24995000);
    }
}
