/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut vec = nums.to_owned();
        let l = vec.len();
        vec.sort();
        vec.dedup();
        l != vec.len()
    }
}
// @lc code=end

