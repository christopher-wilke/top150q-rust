use std::{hash::Hasher, ptr::hash};

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut solution = vec![];

        if nums.len() < 3 {
            return solution;
        }
        
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {

            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }
            
            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let current_sum = nums[i] + nums[left] + nums[right];
                if current_sum == 0 {
                    let v = vec![nums[i], nums[left], nums[right]];
                    solution.push(v);    

                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    
                    left += 1;
                    right -= 1;
                } else {
                    if current_sum > 0 {
                        right -= 1;
                    } else {
                        left += 1;
                    }
                }
            }
        }
        
        solution
    }
}

fn main() {
    let nums = [-1,0,1,2,-1,-4].to_vec();
    let solution = Solution::three_sum(nums);
    println!("{:?}", solution);
}
