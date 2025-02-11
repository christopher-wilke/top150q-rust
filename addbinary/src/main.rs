use std::collections::VecDeque;

pub struct Solution;

impl Solution {

    // pub fn calc(a: char, b: char) -> (char, bool) {
    //     match (a, b) {
    //         ('1', '1') => ('0', true),
    //         ('1', '0') | ('0', '1') => ('1', false),
    //         _ => ('0', false)
    //     }
    // }
    
    pub fn add_binary(a: String, b: String) -> String {
        let mut a_chars = a.chars().collect::<Vec<char>>();
        let mut b_chars = b.chars().collect::<Vec<char>>();

        let mut result: VecDeque<char> = VecDeque::new();

        let mut i: i32 = a.len() as i32 - 1;
        let mut j: i32 = b.len() as i32 - 1;
        let counter = std::cmp::max(i, j);
        let mut rem = '0';

        while counter >= 0 {
            println!("{i} {j}");    
            i -= 1;
            j -= 1;
        }
        

        format!("")
    }
}

fn main() {
    let a = "1".to_string();
    let b = "11".to_string();
    let solution = Solution::add_binary(a, b);
    println!("{solution}");
}
