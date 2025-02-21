pub struct Solution;

impl Solution {

    // Greey, brute-force algorithm
    // Time Complexity: O(n)
    // Space Complexity: O(n) since we need to store every state
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() > 0 {
            let target_idx = nums.len() as i32 - 1;
            let mut farthest = nums[0];

            if nums.len() == 1 {
                return true;
            }

            loop {
                println!("farthest is now on idx: {}", farthest);
                if farthest >= target_idx {
                    return true;
                }
                if nums[farthest as usize] == 0 {
                    return false;
                } else {
                    farthest += nums[farthest as usize];    
                }
            }
        }
        false
    }
}

fn main() {
    let jumps = [2, 5, 0, 0].to_vec();
    println!("jumps={:?}", jumps);
    let can_jump = Solution::can_jump(jumps);
    println!("{:?}", can_jump);
}
