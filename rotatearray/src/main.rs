use std::collections::VecDeque;

pub struct Solution;

impl Solution {

    // Time Complexity: O(k)
    // Space Complexity: O(n)
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k > nums.len() as i32 {
            let mut k = k;
            while k > 0 {
                let start = nums[0..nums.len() - 1 as usize].to_vec();
                *nums = nums[nums.len() - 1 as usize..].to_vec();
                nums.extend(start);
                k -= 1;
            }
        } else {
            let start = nums[0..k as usize].to_vec();
            *nums = nums[k as usize..].to_vec();
            nums.extend(start);
        }
    }
}

fn main() {
    let mut nums = [1, 2, 3, 4, 5, 6, 7].to_vec();
    Solution::rotate(&mut nums, 3);
    println!("{:?}", nums);
}
