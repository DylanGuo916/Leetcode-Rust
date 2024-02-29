/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut ret: bool = true;
        if x >= 0 {
            let s: String = x.to_string();
            let l = s.len();
            for i in 0..l/2 {
                if s.chars().nth(i) != s.chars().nth(l-1-i) {
                    ret = false;
                    break;
                }
            }
        } else {
            ret = false;
        }
        ret
    }
}
// @lc code=end

