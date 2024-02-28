/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根 
 */

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;
        let xx:i64 = x as i64;
        loop {
            if left > right {
                break;
            }

            let mid = left + (right - left) / 2;

            let mut target: i64 = mid as i64 * mid as i64;

            if target == xx {
                return mid;
            } else if target > xx {
                right = mid - 1;
            } else {
                target = (mid + 1) as i64 * (mid + 1) as i64;
                if xx < target {
                    return mid;
                }
                left = mid + 1;
            }
        }
        return -1;
    }
}
// @lc code=end

