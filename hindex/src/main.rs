use std::ffi::c_int;

pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut cits = citations;
        cits.sort_by(|a, b| b.cmp(a));
        let mut current_cit_number = 0;

        for c in cits {
            if c > current_cit_number {
                current_cit_number += 1;
            } else {
                break;
            }
        }
        
        current_cit_number
    }
}

fn main() {
    let citations = [3,0,6,1,5].to_vec();
    let h_index = Solution::h_index(citations);
    println!("h_index={h_index}");
}
