pub struct Solution;

impl Solution {
    pub fn add_one(digits: Vec<i32>) -> Vec<i32> {

        let mut digits = digits;
        let mut pointer = digits.len() as i32 - 1;
        
        while pointer >= 0 {
            let current = digits.get_mut(pointer as usize).unwrap();
            if current != &9 {
                *current += 1;
                break;
            } else {
               *current = 0; 
            }
            pointer -= 1;
        }

        if digits.iter().sum::<i32>() == 0 {
            digits[0] = 1;
            digits.push(0);   
        }
        
        digits
    }
}

fn main() {
    let mut nums = vec![1, 2, 3];
    let add_one = Solution::add_one(nums);
    println!("{:?}", add_one);
}
