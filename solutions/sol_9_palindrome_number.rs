pub struct Solution;

/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let str_of_x: Vec<char> = x.to_string().chars().collect();
        let len = str_of_x.len();
        let mut i = 0;
        while i < len / 2 {
            if str_of_x[i] != str_of_x[len - i - 1] {
                return false;
            }
            i += 1;
        }
        true
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        assert_eq!(Solution::is_palindrome(-131), false);
        assert_eq!(Solution::is_palindrome(131), true);
        assert_eq!(Solution::is_palindrome(132), false);
    }
}
