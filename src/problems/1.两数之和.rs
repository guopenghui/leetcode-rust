/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
        nums.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        let mut i = 0;
        let mut j = nums.len() - 1;

        while nums[i].1 + nums[j].1 != target {
            if nums[i].1 + nums[j].1 > target {
                j -= 1;
            } else {
                i += 1;
            }
        }

        return vec![nums[i].0 as i32, nums[j].0 as i32];
    }
}
// @lc code=end
#[test]
fn base_test() {
    crate::batch_test!(
        Solution::two_sum:
        (vec![2,7,11,15], 9) => vec![0,1],
        (vec![3,2,4], 6) => vec![1,2],
        (vec![3,3], 6) => vec![0,1],
    );
}
