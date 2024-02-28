/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        for i in 0..n {
            nums1[(m + i) as usize] = nums2[i as usize];
        }
        nums1.sort();
    }
}
// @lc code=end

