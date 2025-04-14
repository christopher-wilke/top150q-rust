pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut right = n - 1;
        let overall_sum: i32 = nums[0..right + 1].iter().sum();
        if overall_sum < target {
            return 0;
        }
        let mut window_size = n as i32;
        let mut current_left_sum = 0;
        
        for left in 0..n {
            if nums[left] >= target {
                return 1;
            }
            let mut current_sum: i32 = overall_sum - current_left_sum;
            while left < right && current_sum >= target {
                let current_size = right - left;
                if (current_size as i32 + 1) < window_size {
                    window_size = current_size as i32 + 1;
                }
                right -= 1;
                current_sum -= nums[right + 1];
            }
            right = n - 1;
            current_left_sum += nums[left];
        }
        window_size
    }
}

fn main() {
    let left = 3;
    let right = 5;
    println!("{}", left.min(right));
    // let nums = [2,3,1,2,4,3].to_vec();
    // println!("nums={:?}", nums);
    // let target = 7;
    // let solution = Solution::min_sub_array_len(target, nums);
    // println!("{solution}");
}
