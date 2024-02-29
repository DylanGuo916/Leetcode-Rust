/*
 * @lc app=leetcode.cn id=58 lang=rust
 *
 * [58] 最后一个单词的长度
 */

// @lc code=start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let v: Vec<&str> = s.trim().rsplit(' ').collect();
        if v.len() == 0 {
            0
        } else {
            v[0].len() as i32
        }
    }
}
// @lc code=end

