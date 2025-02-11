pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut tmp = x;
        let mut s = 0;
        while tmp != 0 {
            s = s*10 + tmp%10;
            tmp /= 10;
        }
        x == s
    }
}

fn main() {
    let num = 121;
    let solution =  Solution::is_palindrome(num);
    println!("{:?}", solution);
}
