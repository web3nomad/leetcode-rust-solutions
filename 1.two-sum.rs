/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // solve leetcode two sum problem in O(n)
        // time complexity and O(n) space complexity
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![*map.get(&complement).unwrap() as i32, i as i32];
            }
            map.insert(num, i);
        }
        panic!("No two sum solution");
    }
}
// @lc code=end
