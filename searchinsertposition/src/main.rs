pub struct Solution;

impl Solution {

    // pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //     let mut left = 0;
    //     let mut right = nums.len() as i32 - 1; // Use i32 to prevent overflow

    //     while left <= right {  // Change to `<=`
    //         let middle = left + (right - left) / 2;

    //         if nums[middle as usize] == target {  // Convert `middle` to usize
    //             println!("found it :)");
    //             return middle; 
    //         } else if nums[middle as usize] < target {
    //             println!("left=middle+1");
    //             left = middle + 1;
    //         } else {
    //             println!("right=middle-1");  // Change to `right = middle - 1`
    //             right = middle - 1;
    //         }
    //     }
    //     println!("not found. Correct insert pos: {}", left);
    //     left  // Left will be the correct insert position
    // }
    //

    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: i32 = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid = (left+right) / 2;
            if nums[mid as usize] == target {
                return mid as i32;
            } else if target < nums[mid as usize] {
                right = mid - 1;                
            } else {
                left = mid + 1;
            }
        }
        left as i32
    }
}


fn main() {
    let nums = [1,3,5,6].to_vec();
    let target = 0;
    Solution::search_insert(nums, target);
}
