pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left: usize = 0;
        let mut right: usize = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] == target {
                return vec![left as i32 + 1, right as i32 + 1];
            } else if numbers[left] + numbers[right] > target {
                right -= 1;
            } else {
                left += 1;
            }
        }
        
        vec![]       
    }
}

fn main() {
    let numbers = [2, 3, 4].to_vec();
    let target = 6;
    println!("numbers={:?}", numbers);
    println!("target={target}");
    let res = Solution::two_sum(numbers, target);
    println!("{:?}", res);
}
