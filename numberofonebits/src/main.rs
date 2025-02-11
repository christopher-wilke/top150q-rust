pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let v: Vec<char> = format!("{:b}", n).chars().collect();
        let c = v.iter().fold(
            0,
            |acc, num|
            if num == &'1' {
                acc + 1
            } else {
                acc
            } 
        );
        c
    }
}

fn main() {
    let num = 11;
    let sol = Solution::hamming_weight(num);
    println!("{}", sol);
}
