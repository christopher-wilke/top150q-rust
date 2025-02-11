pub struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut v: Vec<char> = format!("{:032b}", x).chars().collect();
        let rev = v.reverse();
        let num = Self::binary_vec_to_u32(v); 
        num
    }

    fn binary_vec_to_u32(chars: Vec<char>) -> u32 {
        chars.iter()
            .fold(0, |acc, &c| (acc << 1) | (c as u32 - '0' as u32))
    }
}


fn main() {
    let input: u32  =  0b00000010100101000001111010011100;
    let response = Solution::reverse_bits(input);
    println!("{}", response);
}
