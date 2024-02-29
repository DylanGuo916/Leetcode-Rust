/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::from(digits);
        for i in (0..result.len()).rev() {
            if result[i] != 9 {
                result[i] += 1;
                return result
            } else {
                result[i] = 0
            }
        }
        result.insert(0, 1);
        result
    }
}
// @lc code=end

