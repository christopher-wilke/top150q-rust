use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        let mut q: VecDeque<i32>  = VecDeque::new();
        q.push_back(0);
        let mut height = 0;
        let shortest = nums.len() as i32 - 1;
        let mut visited = vec![false; len as usize];

        while !q.is_empty() {
            for _ in 0..q.len() as i32 {
                if let Some(idx) = q.pop_front() {
                    if !visited[idx as usize] {
                        if idx == nums.len() as i32 - 1 && height < shortest {
                            return height;
                        }
                        for i in 1..=nums[idx as usize] {
                            let jump_idx = idx + i;
                            if jump_idx < len {
                                q.push_back(idx + i);
                            }
                        }
                        visited[idx as usize] = true;   
                    }
                }    
            }
            height += 1;         
        }
        shortest
    }
}

fn main() {
    let nums = [2, 3, 0, 1, 4].to_vec();
    println!("nums={:?}", nums);
    let jumps = Solution::jump(nums);
    println!("min_jumps={}", jumps);
}
