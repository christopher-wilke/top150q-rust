pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {

        if x == 0 {
            return 1;
        }
        
        for i in 1..x {
            if i*i == x {
                return i;
            } else {
                if (i*i) < x && (i+1)*(i+1) > x {
                    return i;
                }
            }
        }
        1
    }
}

fn main() {
    let x = 100;
    let s = Solution::my_sqrt(x);
    println!("{}", s);
}
