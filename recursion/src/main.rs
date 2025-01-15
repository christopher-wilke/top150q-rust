struct Solution;

impl Solution {

    // n! = n * (n-1)
    pub fn faculty_recursive(i: i32) -> i32 {
        if i == 1 {
            return 1;
        }

        return i * Self::faculty_recursive(i-1)
    }
    
    pub fn faculty_iterative(mut i: i32) -> i32 {
        let mut res = 1; 
        while i > 1 {
            res *= i;
            i -= 1;
        }
        res
    }
}

fn main() {
    let faculty = 3;
    println!("{}", Solution::faculty_recursive(faculty));
}
