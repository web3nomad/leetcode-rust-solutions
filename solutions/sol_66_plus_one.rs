pub struct Solution;



/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 */

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut i = digits.len();
        let mut p = 1;
        while i > 0 {
            i -= 1;
            let d = digits[i] + p;
            result.insert(0, d % 10);
            p = d / 10;
        }
        if p > 0 {
            result.insert(0, p);
        }
        result
    }
}
// @lc code=end



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1,0]);
    }
}
