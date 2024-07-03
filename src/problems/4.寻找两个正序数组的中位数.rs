/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
struct Solution;
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut n1, mut n2) = (nums1.as_slice(), nums2.as_slice());

        while n1.len() + n2.len() > 10 {
            // 如果长度相差较多，直接删除长数组两侧的无关元素
            if n1.len() > n2.len() {
                std::mem::swap(&mut n1, &mut n2);
            }
            if 2 * n1.len() < n2.len() {
                let trim = n2.len() - n1.len() - 1;
                n2 = &n2[trim / 2..n2.len() - trim / 2];
                continue;
            }

            let median_1 = Self::get_median(n1);
            let median_2 = Self::get_median(n2);
            if median_1 == median_2 {
                return median_1;
            }
            if median_1 > median_2 {
                std::mem::swap(&mut n1, &mut n2);
            }

            let trim = (std::cmp::min(n1.len(), n2.len()) + 1) / 2 - 1;
            n1 = &n1[trim..];
            n2 = &n2[..n2.len() - trim];
        }

        Self::merge_median(n1, n2)
    }

    fn merge_median(nums1: &[i32], nums2: &[i32]) -> f64 {
        let mut merged = MergeIter::new(nums1, nums2);
        let k = (nums1.len() + nums2.len() - 1) / 2;

        let a = merged.nth(k).unwrap();
        if (nums1.len() + nums2.len()) % 2 == 0 {
            let b = merged.next().unwrap();
            (a + b) as f64 / 2.0
        } else {
            a as f64
        }
    }

    #[inline]
    fn get_median(nums: &[i32]) -> f64 {
        let len = nums.len();
        if len % 2 == 0 {
            (nums[len / 2 - 1] + nums[len / 2]) as f64 / 2.0
        } else {
            nums[len / 2] as f64
        }
    }
}

struct MergeIter<'a> {
    nums1: &'a [i32],
    nums2: &'a [i32],
    i: usize,
    j: usize,
}

impl<'a> MergeIter<'a> {
    fn new(nums1: &'a [i32], nums2: &'a [i32]) -> Self {
        Self {
            nums1,
            nums2,
            i: 0,
            j: 0,
        }
    }
}

impl Iterator for MergeIter<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { nums1, nums2, i, j } = self;

        if *i < nums1.len() && *j < nums2.len() {
            let next = if nums1[*i] < nums2[*j] {
                *i += 1;
                nums1[*i - 1]
            } else {
                *j += 1;
                nums2[*j - 1]
            };
            Some(next)
        } else if *i < nums1.len() {
            *i += 1;
            Some(nums1[*i - 1])
        } else if *j < nums2.len() {
            *j += 1;
            Some(nums2[*j - 1])
        } else {
            None
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    static FUNC: fn(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 = Solution::find_median_sorted_arrays;
    #[test]
    fn merge_median_test() {
        assert_eq!(Solution::merge_median(&[1, 3], &[2, 4]), 2.5);
    }

    #[test]
    fn test_1() {
        assert_eq!(FUNC(vec![1, 3], vec![2]), 2.0);
    }

    #[test]
    fn test_2() {
        assert_eq!(FUNC(vec![1, 2], vec![3, 4]), 2.5);
    }

    #[test]
    fn test_3() {
        assert_eq!(
            FUNC(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![1, 2, 3, 4, 5, 6]),
            4.0
        );
    }

    #[test]
    fn test_4() {
        assert_eq!(
            FUNC(
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22],
                vec![0, 6]
            ),
            10.5
        );
    }

    #[test]
    fn test_5() {
        assert_eq!(FUNC(vec![], vec![1]), 1.0);
    }
}
