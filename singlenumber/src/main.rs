pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums
            .iter()
            .fold(0, |acc, current| acc ^ current)
    }
}

fn main() {
    let nums = vec![2, 4, 4, 2, 1];
    let solution = Solution::single_number(nums);
    println!("{}", solution);
}
